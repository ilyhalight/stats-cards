use crate::api::github::GraphQLResponse;
use crate::api::{github, wakatime};
use crate::data::config::CONFIG;
use crate::data::language::get_lang_color;
use crate::data::theme::{Theme, ThemeData};
use crate::prepared_templates::{PreparedTemplate, gh_handle_error_template};
use crate::{templates, utils};

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use itertools::Itertools;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MAX_BAR_WIDTH: f32 = 275.0;

#[derive(Deserialize, Serialize)]
pub struct Params {
    username: String,
    theme: Option<Theme>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageStat {
    name: String,
    color: String,
    percent: f32,
}

#[derive(Template)]
#[template(path = "compact/languages.html")]
pub struct CompactLanguagesTemplate {
    name: String,
    stats_bar: String,
    bar_legend: String,
    theme_data: ThemeData,
}

async fn get_top_langs_by_waka_intl(
    cache: Cache<String, String>,
    username: &String,
) -> Result<Vec<LanguageStat>, PreparedTemplate> {
    let cache_key = format!("wakatime:langs:{username}");
    if let Some(cached) = cache.get(&cache_key).await {
        let langs = serde_json::from_str(&cached).unwrap();
        return Ok(langs);
    }

    let stats = wakatime::get_stats(username).await;
    if !stats.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    let stats_data = utils::wakatime::unwrap_stats_response(stats.unwrap())?;
    let languages = stats_data.data.languages;
    let first_languages = match languages.first_chunk::<6>() {
        Some(langs) => langs,
        None => {
            return Err(PreparedTemplate::FailedFindStats);
        }
    };

    let max_percent = first_languages
        .iter()
        .fold(0.0, |acc, val| acc + val.base.percent);
    let top_langs: Vec<LanguageStat> = first_languages
        .iter()
        .map(|lang| LanguageStat {
            name: lang.base.name.clone(),
            color: get_lang_color(&lang.base.name),
            percent: 100.0 / (max_percent / lang.base.percent),
        })
        .collect();

    let cache_body = serde_json::to_string(&top_langs).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(top_langs)
}

async fn get_top_langs_by_github_intl(
    cache: Cache<String, String>,
    username: &String,
) -> Result<Vec<LanguageStat>, PreparedTemplate> {
    let cache_key = format!("github:langs:{username}");
    if let Some(cached) = cache.get(&cache_key).await {
        let langs = serde_json::from_str(&cached).unwrap();
        return Ok(langs);
    }

    let stats = github::get_languages(username).await;
    if !stats.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    let languages_raw_data = match stats.unwrap() {
        GraphQLResponse::Failed(err) => return Err(gh_handle_error_template(err)),
        GraphQLResponse::Valid(res) => match res.data.user {
            None => return Err(PreparedTemplate::FailedFindUser),
            Some(user_data) => user_data.repositories.nodes,
        },
    };

    if languages_raw_data.len() == 0 {
        return Err(PreparedTemplate::FailedFindStats);
    }

    let mut langs_data: HashMap<String, i64> = HashMap::new();
    languages_raw_data
        .iter()
        .filter(|&lang| lang.languages.edges.len() > 0)
        .flat_map(|lang| &lang.languages.edges)
        .for_each(|lang| {
            langs_data
                .entry(lang.node.name.clone())
                .and_modify(|count| {
                    *count += lang.size as i64;
                })
                .or_insert(lang.size as i64);
        });

    let first_languages = langs_data
        .iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1))
        .take(6)
        .collect::<Vec<(&String, &i64)>>();

    let max_bytes = first_languages
        .iter()
        .fold(0 as i64, |acc, val| acc + val.1) as f64;
    let top_langs: Vec<LanguageStat> = first_languages
        .iter()
        .map(|(name, size)| {
            let lang_name = name.to_string();
            let lang_size = **size as f64;
            let percent = (lang_size / max_bytes * 100.0) as f32;

            LanguageStat {
                name: lang_name,
                color: get_lang_color(name),
                percent,
            }
        })
        .collect();

    let cache_body = serde_json::to_string(&top_langs).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(top_langs)
}

pub fn render_top_langs(
    username: String,
    theme: Theme,
    top_langs_res: Result<Vec<LanguageStat>, PreparedTemplate>,
) -> Response {
    if !top_langs_res.is_ok() {
        return top_langs_res.unwrap_err().render();
    }

    let theme_data = theme.get_data();
    let stats = top_langs_res.unwrap();
    let mut bar_start_x = 20.0;
    let mut column_start_y = 93;

    let bar_data: Vec<String> = stats
        .iter()
        .map(|stat| {
            let stat_percent = stat.percent / 100.0;
            let block_width = MAX_BAR_WIDTH * stat_percent;
            let element = format!(
                r##"<rect mask="url(#stats_mask)" x="{bar_start_x:.2}" y="61" width="{block_width:.2}" height="10" fill="{0}" />"##,
                stat.color
            );

            bar_start_x += block_width;

            element
        })
        .collect();

    let bar_legend: Vec<String> = stats
        .iter()
        .enumerate()
        .map(|(idx, stat)| {
            let start_x = if idx < 3 { 20 } else { 175 };
            let text_x = start_x + 18;
            let start_y = column_start_y;
            let text_y = start_y + 11;
            let element = format!(
                r##"
                <g>
                    <rect x="{start_x}" y="{start_y}" width="12" height="12" rx="6" fill="{0}" />
                    <text x="{text_x}" y="{text_y}" fill="{1}" class="stat-text">{2} {3:.2}%</text>
                </g>
            "##,
                stat.color, &theme_data.text, stat.name, stat.percent,
            );

            column_start_y = if idx == 2 { 93 } else { column_start_y + 24 };

            element
        })
        .collect();

    let template = CompactLanguagesTemplate {
        name: username,
        stats_bar: bar_data.join("\n"),
        bar_legend: bar_legend.join("\n"),
        theme_data,
    };
    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<CompactLanguagesTemplate>::into_response(svg_template)
}

pub async fn get_waka_top_langs(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let username = params.username;
    let theme = params.theme.unwrap_or(CONFIG.default_theme.clone());
    let top_langs_res = get_top_langs_by_waka_intl(cache, &username).await;
    render_top_langs(username, theme, top_langs_res)
}

pub async fn get_github_top_langs(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let username = params.username;
    let theme = params.theme.unwrap_or(CONFIG.default_theme.clone());
    let top_langs_res = get_top_langs_by_github_intl(cache, &username).await;
    render_top_langs(username, theme, top_langs_res)
}
