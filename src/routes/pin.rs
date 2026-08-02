use std::vec;

use crate::api::github::{
    self, Gist, GistResponse as GithubGistResponse, Repository, RestResponse,
};
use crate::api::huggingface::{self};
use crate::data::config::CONFIG;
use crate::data::language::get_lang_color;
use crate::data::theme::{Theme, ThemeData};
use crate::prepared_templates::{
    PreparedTemplate, gh_handle_error_template, hf_handle_error_template,
};
use crate::templates;
use crate::utils::svg::wrap_text;
use crate::utils::utils::fmt_num;
use crate::{
    api::huggingface::{
        DatasetResponse as HFDatasetResponse, ModelResponse as HFModelResponse,
        RepoData as HFRepoData, RepoResponse as HFRepoResponse, SpaceResponse as HFSpaceResponse,
    },
    utils::svg::calc_width,
};

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use moka::future::Cache;
use serde::{Deserialize, Serialize};

const MAX_PIN_WIDTH: usize = 400;

#[derive(Deserialize, Serialize)]
pub struct HFParams {
    username: String,
    repo: String,
    theme: Option<Theme>,
    #[serde(rename = "type")]
    typename: HFPinIcon,
    show_owner: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct GHParams {
    username: String,
    repo: String,
    theme: Option<Theme>,
    show_owner: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct GistParams {
    id: String,
    theme: Option<Theme>,
    show_owner: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HFPinIcon {
    Model,
    Dataset,
    Space,
}

impl PartialEq<&str> for HFPinIcon {
    fn eq(&self, other: &&str) -> bool {
        match self {
            HFPinIcon::Model => *other == "model",
            HFPinIcon::Dataset => *other == "dataset",
            HFPinIcon::Space => *other == "space",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HFTag {
    name: String,
    width: usize,
    translate_x: usize,
    visible: bool,
}

#[derive(Template)]
#[template(path = "compact/pin/huggingface.html")]
pub struct HFPinTemplate<'a> {
    name: String,
    desc: String,
    repo_text: String,
    likes: &'a String,
    downloads: Option<&'a String>,
    icon: HFPinIcon,
    tags: Vec<HFTag>,
    theme_data: ThemeData,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GHPinIcon {
    #[serde(rename = "repo")]
    Repo,
    #[serde(rename = "gist")]
    Gist,
}

impl PartialEq<&str> for GHPinIcon {
    fn eq(&self, other: &&str) -> bool {
        match self {
            GHPinIcon::Repo => *other == "repo",
            GHPinIcon::Gist => *other == "gist",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GHLangText {
    pub name: String,
    pub width: usize,
    pub color: String,
}

#[derive(Template)]
#[template(path = "compact/pin/github.html")]
pub struct GHPinTemplate<'a> {
    name: String,
    desc: String,
    repo_text: String,
    icon: GHPinIcon,
    rows: Vec<String>,
    language: Option<GHLangText>,
    stars: Option<&'a String>,
    forks: Option<&'a String>,
    svg_height: i32,
    meta_y_indent: i32,
    meta_counters_x_indent: usize,
    forks_counter_x_indent: usize,
    theme_data: ThemeData,
}

pub async fn huggingface_get_data(
    username: &String,
    repo: &String,
    typename: &HFPinIcon,
) -> Result<HFRepoData, PreparedTemplate> {
    let data = match typename {
        HFPinIcon::Model => huggingface::get_model(username, repo)
            .await
            .map(HFRepoResponse::Model),
        HFPinIcon::Dataset => huggingface::get_dataset(username, repo)
            .await
            .map(HFRepoResponse::Dataset),
        HFPinIcon::Space => huggingface::get_space(username, repo)
            .await
            .map(HFRepoResponse::Space),
    };
    if !data.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    match data.unwrap() {
        HFRepoResponse::Model(HFModelResponse::Valid(res)) => Ok(HFRepoData::Model(res)),
        HFRepoResponse::Dataset(HFDatasetResponse::Valid(res)) => Ok(HFRepoData::Dataset(res)),
        HFRepoResponse::Space(HFSpaceResponse::Valid(res)) => Ok(HFRepoData::Space(res)),
        HFRepoResponse::Model(HFModelResponse::Failed(err))
        | HFRepoResponse::Space(HFSpaceResponse::Failed(err))
        | HFRepoResponse::Dataset(HFDatasetResponse::Failed(err)) => {
            Err(hf_handle_error_template(err))
        }
    }
}

pub async fn get_huggingface_pin_impl(
    cache: Cache<String, String>,
    username: &String,
    repo: &String,
    typename: &HFPinIcon,
) -> Result<HFRepoData, PreparedTemplate> {
    if username.is_empty() || repo.is_empty() {
        return Err(PreparedTemplate::FailedFindRepo);
    }

    let cache_key = format!("huggingface:{:?}:{username}:{repo}", typename);
    if let Some(cached) = cache.get(&cache_key).await {
        let data = serde_json::from_str(&cached).unwrap();
        return Ok(data);
    }

    let data = huggingface_get_data(username, repo, typename).await;
    let result = match data {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let cache_body = serde_json::to_string(&result).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(result)
}

pub fn render_huggingface_pin(
    username: String,
    repo: String,
    show_owner: bool,
    theme: Theme,
    pin_data: Result<HFRepoData, PreparedTemplate>,
) -> Response {
    let raw_data = match pin_data {
        Ok(data) => data,
        Err(err) => return err.render(),
    };

    let mut raw_tags: Vec<String> = vec![];
    let downloads_raw: &Option<String> = &raw_data.get_downloads_count();
    let downloads: Option<&String> = downloads_raw.as_ref();
    if let HFRepoData::Model(model) = &raw_data {
        if let Some(model_type) = model
            .config
            .as_ref()
            .and_then(|config| config.model_type.clone())
        {
            raw_tags.push(model_type);
        }

        if let Some(pipeline_tag) = &model.pipeline_tag {
            raw_tags.push(pipeline_tag.to_string());
        }
    }

    if let HFRepoData::Dataset(dataset) = &raw_data {
        if let Some(task_category) = &dataset
            .base
            .card_data
            .task_categories
            .as_ref()
            .and_then(|categories| categories.first())
        {
            raw_tags.push((*task_category).clone())
        }
    }

    if let HFRepoData::Space(space) = &raw_data {
        let running_on = space.runtime.hardware.current.to_string();
        raw_tags.push(format!("Running on {running_on}"));
    }

    let icon = match &raw_data {
        HFRepoData::Model(_) => HFPinIcon::Model,
        HFRepoData::Dataset(_) => HFPinIcon::Dataset,
        HFRepoData::Space(_) => HFPinIcon::Space,
    };

    if let Some(license) = &raw_data.get_license() {
        if license != "other" {
            raw_tags.push(license.to_uppercase());
        }
    }

    let likes = &raw_data.get_likes();
    if raw_tags.len() == 0 {
        if let Some(tag) = &raw_data.get_repo_tags().first() {
            raw_tags.push((*tag).clone());
        }
    }

    let mut translate_x: usize = 0;
    let tags: Vec<HFTag> = raw_tags
        .iter()
        .map(|tag| {
            let width = calc_width(tag, 13.0) + 8;
            let tag_translate_x = translate_x;
            translate_x += width + 10;
            return HFTag {
                name: tag.clone(),
                width,
                translate_x: tag_translate_x,
                visible: width + tag_translate_x <= MAX_PIN_WIDTH,
            };
        })
        .filter(|tag| tag.visible)
        .collect();

    let repo_text = if show_owner {
        raw_data.get_id()
    } else {
        repo.clone()
    };

    let theme_data = theme.get_data();
    let template = HFPinTemplate {
        name: username,
        desc: repo,
        repo_text,
        downloads,
        likes,
        icon,
        tags,
        theme_data,
    };

    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<HFPinTemplate>::into_response(svg_template)
}

pub async fn github_get_data(
    username: &String,
    repo: &String,
) -> Result<Repository, PreparedTemplate> {
    let data = github::get_repo(username, repo).await;
    if !data.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    match data.unwrap() {
        RestResponse::Failed(res) => return Err(gh_handle_error_template(res)),
        RestResponse::Valid(res) => Ok(res),
    }
}

pub async fn get_github_pin_impl(
    cache: Cache<String, String>,
    username: &String,
    repo: &String,
) -> Result<Repository, PreparedTemplate> {
    if username.is_empty() || repo.is_empty() {
        return Err(PreparedTemplate::FailedFindRepo);
    }

    let cache_key = format!("github:repo:{username}:{repo}");
    if let Some(cached) = cache.get(&cache_key).await {
        let data = serde_json::from_str(&cached).unwrap();
        return Ok(data);
    }

    let data = github_get_data(username, repo).await;
    let result = match data {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let cache_body = serde_json::to_string(&result).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(result)
}

pub fn render_github_pin(
    username: String,
    repo: String,
    show_owner: bool,
    theme: Theme,
    repo_data: Result<Repository, PreparedTemplate>,
) -> Response {
    let raw_data = match repo_data {
        Ok(data) => data,
        Err(err) => return err.render(),
    };

    let repo_text = if show_owner {
        format!("{username}/{repo}")
    } else {
        repo.clone()
    };

    let rows = match raw_data.description {
        Some(desc) => wrap_text(&desc, 13.0, 365),
        None => vec!["No description provided".to_string()],
    };

    let stars_pretty = fmt_num(raw_data.stargazers_count as i32);
    let stars = if raw_data.stargazers_count == 0 {
        None
    } else {
        Some(&stars_pretty)
    };
    let forks_pretty = fmt_num(raw_data.forks_count as i32);
    let forks = if raw_data.forks_count == 0 {
        None
    } else {
        Some(&forks_pretty)
    };

    let language = match raw_data.language {
        Some(lang) => Some(GHLangText {
            color: get_lang_color(&lang),
            width: calc_width(&lang, 13.0),
            name: lang,
        }),
        None => None,
    };
    let theme_data = theme.get_data();
    let is_single_text_row = rows.len() == 1;

    let svg_height = get_svg_height(is_single_text_row);
    let meta_y_indent = get_meta_y_indent(is_single_text_row);
    let template = GHPinTemplate {
        name: username,
        desc: repo,
        repo_text,
        icon: GHPinIcon::Repo,
        rows,
        stars,
        forks,
        svg_height,
        meta_y_indent,
        meta_counters_x_indent: if let Some(lang) = &language {
            lang.width + 35
        } else {
            0
        },
        forks_counter_x_indent: if let Some(stars) = &stars {
            calc_width(stars, 13.0) + 35
        } else {
            0
        },
        language,
        theme_data,
    };
    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<GHPinTemplate>::into_response(svg_template)
}

pub async fn get_huggingface_pin(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<HFParams>,
) -> Response {
    let theme = params.theme.unwrap_or(CONFIG.default_theme.clone());
    let username = params.username;
    let repo = params.repo;
    let typename = params.typename;
    let show_owner = if let Some(show_owner) = params.show_owner {
        show_owner
    } else {
        false
    };
    let repo_data = get_huggingface_pin_impl(cache, &username, &repo, &typename).await;
    render_huggingface_pin(username, repo, show_owner, theme, repo_data)
}

pub async fn get_github_repo_pin(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<GHParams>,
) -> Response {
    let theme = params.theme.unwrap_or(CONFIG.default_theme.clone());
    let username = params.username;
    let repo = params.repo;
    let show_owner = if let Some(show_owner) = params.show_owner {
        show_owner
    } else {
        false
    };

    let repo_data = get_github_pin_impl(cache, &username, &repo).await;
    render_github_pin(username, repo, show_owner, theme, repo_data)
}

pub async fn gist_get_data(id: &String) -> Result<Gist, PreparedTemplate> {
    let data = github::get_gist(id).await;

    if !data.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    match data.unwrap() {
        GithubGistResponse::Failed(res) => return Err(gh_handle_error_template(res)),
        GithubGistResponse::Valid(res) => match res.data.viewer.gist {
            None => Err(PreparedTemplate::FailedFindRepo),
            Some(gist_data) => Ok(gist_data),
        },
    }
}

pub async fn get_gist_pin_impl(
    cache: Cache<String, String>,
    id: &String,
) -> Result<Gist, PreparedTemplate> {
    if id.is_empty() {
        return Err(PreparedTemplate::FailedFindRepo);
    }

    let cache_key = format!("github:gist:{id}");
    if let Some(cached) = cache.get(&cache_key).await {
        let data = serde_json::from_str(&cached).unwrap();
        return Ok(data);
    }

    let data = gist_get_data(id).await;
    let result = match data {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let cache_body = serde_json::to_string(&result).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(result)
}

fn get_svg_height(is_single_text_row: bool) -> i32 {
    if is_single_text_row { 100 } else { 120 }
}

fn get_meta_y_indent(is_single_text_row: bool) -> i32 {
    if is_single_text_row { 70 } else { 90 }
}

pub fn render_github_gist(
    gist_id: String,
    show_owner: bool,
    theme: Theme,
    repo_data: Result<Gist, PreparedTemplate>,
) -> Response {
    let raw_data = match repo_data {
        Ok(data) => data,
        Err(err) => return err.render(),
    };

    let mut language: Option<GHLangText> = None;
    let mut repo = gist_id;
    let username = raw_data.owner.login;
    let gist_file = raw_data.files.iter().max_by_key(|file| file.size);
    if let Some(file) = gist_file {
        language = Some(GHLangText {
            width: calc_width(&file.language.name, 13.0),
            color: get_lang_color(&file.language.name),
            name: file.language.name.clone(),
        });
        repo = file.name.clone();
    }

    let repo_text = if show_owner {
        format!("{username}/{repo}")
    } else {
        repo.clone()
    };

    let rows = match raw_data.description {
        Some(desc) => wrap_text(&desc, 13.0, 365),
        None => vec!["No description provided".to_string()],
    };

    let stars_pretty = fmt_num(raw_data.stargazer_count as i32);
    let stars = if raw_data.stargazer_count == 0 {
        None
    } else {
        Some(&stars_pretty)
    };
    let forks_count = raw_data.forks.total_count as i32;
    let forks_pretty = fmt_num(forks_count);
    let forks = if forks_count == 0 {
        None
    } else {
        Some(&forks_pretty)
    };

    let theme_data = theme.get_data();
    let is_single_text_row = rows.len() == 1;
    let svg_height = get_svg_height(is_single_text_row);
    let meta_y_indent = get_meta_y_indent(is_single_text_row);
    let template = GHPinTemplate {
        name: username,
        desc: repo,
        repo_text,
        icon: GHPinIcon::Gist,
        rows,
        stars,
        forks,
        svg_height,
        meta_y_indent,
        meta_counters_x_indent: if let Some(lang) = &language {
            lang.width + 35
        } else {
            0
        },
        forks_counter_x_indent: if let Some(stars) = &stars {
            calc_width(stars, 13.0) + 35
        } else {
            0
        },
        language,
        theme_data,
    };
    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<GHPinTemplate>::into_response(svg_template)
}

pub async fn get_github_gist_pin(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<GistParams>,
) -> Response {
    let theme = params.theme.unwrap_or(CONFIG.default_theme.clone());
    let id = params.id;
    let show_owner = if let Some(show_owner) = params.show_owner {
        show_owner
    } else {
        false
    };

    let repo_data = get_gist_pin_impl(cache, &id).await;
    render_github_gist(id, show_owner, theme, repo_data)
}
