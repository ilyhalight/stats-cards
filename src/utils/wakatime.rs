use crate::api::wakatime::{Stats, StatsResponse, SuccessResponse};
use crate::prepared_templates::PreparedTemplate;

pub fn unwrap_stats_response(
    stats: StatsResponse,
) -> Result<SuccessResponse<Stats>, PreparedTemplate> {
    match stats {
        StatsResponse::Failed(err) => {
            let err_template = match err.error.as_str() {
                "Not found." => PreparedTemplate::FailedFindUser,
                "Time range not matching user's public stats range." => {
                    PreparedTemplate::FailedFindStats
                }
                _ => PreparedTemplate::Unknown,
            };
            return Err(err_template);
        }
        StatsResponse::NoData(_) => {
            return Err(PreparedTemplate::FailedFindStats);
        }
        StatsResponse::Valid(res) => Ok(res),
    }
}
