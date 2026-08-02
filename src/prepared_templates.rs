use crate::{
    api::{
        github::ErrorResponse as GHErrorResponse, huggingface::ErrorResponse as HFErrorResponse,
    },
    templates::{ErrorTemplate, SVGTemplate},
};

use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum PreparedTemplate {
    FailedFindUser,
    FailedFindRepo,
    FailedFindStats,
    BadCredentials,
    APIRateLimit,
    Unknown,
}

impl PreparedTemplate {
    pub fn render(&self) -> Response {
        let template = match self {
            PreparedTemplate::FailedFindUser => ErrorTemplate {
                first_line: "Failed to find a user.",
                second_line: "Check if it’s spelled correctly",
            },
            PreparedTemplate::FailedFindRepo => ErrorTemplate {
                first_line: "Failed to find a repo.",
                second_line: "Check if it’s spelled correctly",
            },
            PreparedTemplate::FailedFindStats => ErrorTemplate {
                first_line: "Failed to find a user stats.",
                second_line: "Maybe he's inactive",
            },
            PreparedTemplate::BadCredentials => ErrorTemplate {
                first_line: "Bad credentials.",
                second_line: "Problems with service API token",
            },
            PreparedTemplate::APIRateLimit => ErrorTemplate {
                first_line: "Failed to fetch.",
                second_line: "Maybe our API ratelimited :(",
            },
            PreparedTemplate::Unknown => ErrorTemplate {
                first_line: "Unknown API error.",
                second_line: "Let us know about it",
            },
        };

        let svg_template = SVGTemplate(template);
        return SVGTemplate::<ErrorTemplate>::into_response(svg_template);
    }
}

pub fn gh_handle_error_template(err: GHErrorResponse) -> PreparedTemplate {
    if err.message.contains("rate limit exceeded") {
        PreparedTemplate::APIRateLimit
    } else if err.message.contains("Bad credentials") {
        PreparedTemplate::BadCredentials
    } else {
        PreparedTemplate::Unknown
    }
}

pub fn hf_handle_error_template(err: HFErrorResponse) -> PreparedTemplate {
    match err.error.as_str() {
        "Invalid credentials in Authorization header" => PreparedTemplate::BadCredentials,
        "Invalid username or password." => PreparedTemplate::BadCredentials,
        "Repository not found" => PreparedTemplate::FailedFindRepo,
        _ => PreparedTemplate::Unknown,
    }
}
