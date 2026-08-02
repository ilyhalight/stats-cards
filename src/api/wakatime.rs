use lazy_static::lazy_static;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use crate::pub_struct;

lazy_static! {
    static ref REQ_CLIENT: Client = Client::new();
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub enum StatsRange {
    #[serde(rename = "last_7_days")]
    Last7Days,
    #[serde(rename = "last_30_days")]
    Last30Days,
    #[serde(rename = "last_6_months")]
    Last6Months,
    #[serde(rename = "last_year")]
    LastYear,
    #[serde(rename = "all_time")]
    AllTime,
}

// same for editors, categories, languages
pub_struct! { EntryActivity {
    total_seconds: f64,
    digital: String,
    decimal: String,
    text: String,
    hours: i32,
    minutes: i8,
}}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub name: String,
    pub percent: f32,
    #[serde(flatten)]
    pub activity: Option<EntryActivity>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum EntryDetailedSeconds {
    Str(String),
    F64(f64),
}

impl EntryDetailedSeconds {
    pub fn as_f64(&self) -> f64 {
        match self {
            EntryDetailedSeconds::F64(v) => *v,
            EntryDetailedSeconds::Str(s) => s.parse().unwrap_or(0.0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryDetailed {
    #[serde(flatten)]
    pub base: Entry,
    pub ai_coding_seconds: EntryDetailedSeconds,
    pub manual_coding_seconds: EntryDetailedSeconds,
}

pub_struct! { ActivityStats {
    holidays: i32,
    total_seconds: f64,
    total_seconds_including_other_language: f64,
    days_minus_holidays: i32,
    daily_average_including_other_language: f64,
    human_readable_daily_average_including_other_language: String,
    daily_average: f64,
    human_readable_daily_average: String,
    human_readable_total_including_other_language: String,
    human_readable_total: String,
    days_including_holidays: i32,
}}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    pub id: String,
    pub user_id: String,
    pub range: StatsRange,
    pub timeout: u32,
    pub username: String,
    pub writes_only: bool,
    pub status: String, // pending_update | ok
    pub percent_calculated: i8,
    pub human_readable_range: String,

    pub is_up_to_date_pending_future: bool,
    pub is_already_updating: bool,
    pub is_stuck: bool,
    pub is_cached: bool,
    pub is_including_today: bool,
    pub is_up_to_date: bool,
    pub is_coding_activity_visible: bool,
    pub is_language_usage_visible: bool,
    pub is_editor_usage_visible: bool,
    pub is_category_usage_visible: bool,
    pub is_os_usage_visible: bool,

    pub operating_systems: Vec<EntryDetailed>,
    pub editors: Vec<EntryDetailed>,
    pub categories: Vec<Entry>,
    pub languages: Vec<EntryDetailed>,

    #[serde(flatten)]
    pub activity: Option<ActivityStats>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct SuccessResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct ErrorResponse {
    pub error: String,
}

pub_struct! { PrivateStats {
    is_coding_activity_visible: bool,
    is_language_usage_visible: bool,
    is_editor_usage_visible: bool,
    is_category_usage_visible: bool,
    is_os_usage_visible: bool,
    is_up_to_date: bool,
    is_up_to_date_pending_future: bool,
    percent_calculated: i32,
    status: String,
}}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StatsResponse {
    Failed(ErrorResponse),
    Valid(SuccessResponse<Stats>),
    NoData(SuccessResponse<PrivateStats>),
}

pub async fn get_stats(username: &String) -> Result<StatsResponse, Error> {
    let request_url = format!("https://wakatime.com/api/v1/users/{username}/stats/all_time");
    let stats = REQ_CLIENT
        .get(&request_url)
        .send()
        .await?
        .json::<StatsResponse>()
        .await?;

    Ok(stats)
}
