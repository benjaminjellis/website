use color_eyre::eyre;
use thiserror::Error;
use tracing::error;

use crate::templates::InternalServerErrorTemplate;

#[derive(Error, Debug)]
pub(crate) enum WebsiteError {
    #[error(transparent)]
    Report(#[from] eyre::Report),
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error("Failed to format date")]
    DateFormat,
}

impl askama_axum::IntoResponse for WebsiteError {
    fn into_response(self) -> askama_axum::Response {
        error!(?self);
        match self {
            WebsiteError::Sqlx(report) => InternalServerErrorTemplate {
                message: format!("DB Error: {report}"),
            }
            .into_response(),
            WebsiteError::DateFormat => InternalServerErrorTemplate {
                message: "Failed to format date".into(),
            }
            .into_response(),
            WebsiteError::Report(report) => InternalServerErrorTemplate {
                message: format!("DB Error: {report}"),
            }
            .into_response(),
        }
    }
}
