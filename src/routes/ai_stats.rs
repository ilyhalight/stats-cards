use crate::api::wakatime::{self};
use crate::data::config::CONFIG;
use crate::data::theme::{Theme, ThemeData};
use crate::prepared_templates::PreparedTemplate;
use crate::utils::utils::fmt_dur;
use crate::{pub_struct, templates, utils};

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use moka::future::Cache;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Params {
    username: String,
    theme: Option<Theme>,
}

pub_struct! { AIStats {
    percent: u8,
    total_time: String,
    ai_editor: String,
}}

#[derive(Template)]
#[template(path = "compact/ai-stats.html")]
pub struct CompactAiStatsTemplate {
    name: String,
    theme_data: ThemeData,
    stats: AIStats,
    dasharray: String,
}

const PROGRESS_RADIUS: f32 = 40.0;

fn get_entry_total_seconds(entry: Option<&wakatime::Entry>) -> f64 {
    let entry = match entry {
        Some(e) => e,
        None => return 0.0,
    };

    if let Some(activity) = &entry.activity {
        activity.total_seconds
    } else {
        0.0
    }
}

fn get_entry_human_time(entry: Option<&wakatime::Entry>) -> String {
    let entry = match entry {
        Some(e) => e,
        None => return "0s".to_string(),
    };

    if let Some(activity) = &entry.activity {
        activity.text.clone()
    } else {
        "0s".to_string()
    }
}

pub_struct! {AIActivty {
    percent: u8,
    total_time: String,
}}

fn calc_ai_percent(coding_secs: f64, ai_coding_secs: f64) -> u8 {
    if coding_secs > 0.0 {
        (ai_coding_secs / coding_secs * 100.0).round() as u8
    } else {
        0
    }
}

fn calc_entries_total_seconds(entries: &Vec<wakatime::EntryDetailed>, is_ai_seconds: bool) -> f64 {
    entries.iter().fold(0.0, |acc, entry| {
        let secs = if is_ai_seconds {
            entry.ai_coding_seconds.as_f64()
        } else {
            entry.manual_coding_seconds.as_f64()
        };

        acc + secs
    })
}

fn manual_restore_ai_activity(entries: &Vec<wakatime::EntryDetailed>) -> AIActivty {
    let coding_seconds = calc_entries_total_seconds(entries, false);
    let ai_coding_seconds = calc_entries_total_seconds(entries, true);
    let percent = calc_ai_percent(coding_seconds, ai_coding_seconds);
    AIActivty {
        percent,
        total_time: fmt_dur(ai_coding_seconds as u64),
    }
}

fn get_ai_activity(stats_data: &wakatime::SuccessResponse<wakatime::Stats>) -> AIActivty {
    let data = &stats_data.data;
    match data {
        wakatime::Stats {
            is_coding_activity_visible: true,
            is_category_usage_visible: true,
            ..
        } => {
            let coding_data = data.categories.iter().find(|cat| cat.name == "Coding");
            let ai_coding_data = data.categories.iter().find(|cat| cat.name == "AI Coding");
            let ai_coding_seconds = get_entry_total_seconds(ai_coding_data);
            let coding_seconds = get_entry_total_seconds(coding_data);
            let percent = calc_ai_percent(coding_seconds, ai_coding_seconds);

            AIActivty {
                percent,
                total_time: get_entry_human_time(ai_coding_data),
            }
        }
        wakatime::Stats {
            is_os_usage_visible: true,
            ..
        } => manual_restore_ai_activity(&data.operating_systems),
        wakatime::Stats {
            is_editor_usage_visible: true,
            ..
        } => manual_restore_ai_activity(&data.editors),
        wakatime::Stats {
            is_language_usage_visible: true,
            ..
        } => manual_restore_ai_activity(&data.languages),
        _ => AIActivty {
            percent: 0,
            total_time: "N/A".to_string(),
        },
    }
}

fn get_ai_stats(stats_data: wakatime::SuccessResponse<wakatime::Stats>) -> AIStats {
    let ai_activity = get_ai_activity(&stats_data);
    let preferred_ai_editor = &stats_data
        .data
        .editors
        .iter()
        .max_by_key(|editor| editor.ai_coding_seconds.as_f64().to_bits());
    let ai_editor = match preferred_ai_editor {
        Some(editor) => editor.base.name.clone(),
        None => "N/A".to_string(),
    };

    AIStats {
        percent: ai_activity.percent,
        total_time: ai_activity.total_time,
        ai_editor,
    }
}

async fn get_ai_stats_by_waka_intl(
    cache: Cache<String, String>,
    username: &String,
) -> Result<AIStats, PreparedTemplate> {
    let cache_key = format!("wakatime:ai-stats:{username}");
    if let Some(cached) = cache.get(&cache_key).await {
        let ai_stats = serde_json::from_str(&cached).unwrap();
        return Ok(ai_stats);
    }

    let stats = wakatime::get_stats(username).await;
    if !stats.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    let stats_data = utils::wakatime::unwrap_stats_response(stats.unwrap())?;
    let ai_stats = get_ai_stats(stats_data);

    let cache_body = serde_json::to_string(&ai_stats).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(ai_stats)
}

pub fn render_ai_stats(
    username: String,
    theme: Theme,
    ai_stats_res: Result<AIStats, PreparedTemplate>,
) -> Response {
    if !ai_stats_res.is_ok() {
        return ai_stats_res.unwrap_err().render();
    }

    let stats = ai_stats_res.unwrap();
    let theme_data = theme.get_data();
    let circum = 2.0 * std::f32::consts::PI * PROGRESS_RADIUS;
    let filled = circum * (stats.percent as f32 / 100.0);
    let empty = circum - filled;
    let dasharray = format!("{filled} {empty}");
    let template = CompactAiStatsTemplate {
        name: username,
        theme_data,
        stats,
        dasharray,
    };
    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<CompactAiStatsTemplate>::into_response(svg_template)
}

pub async fn get_waka_ai_stats(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let username = params.username;
    let theme = params.theme.unwrap_or(CONFIG.default_theme.clone());
    let ai_stats_res = get_ai_stats_by_waka_intl(cache, &username).await;
    render_ai_stats(username, theme, ai_stats_res)
}
