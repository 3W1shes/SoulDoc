use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::request_id::{generate_request_id, RequestId};

#[derive(Debug, Serialize)]
pub struct AgentErrorBody {
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct AgentEnvelope<T>
where
    T: Serialize,
{
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<AgentErrorBody>,
    pub request_id: String,
}

impl<T> AgentEnvelope<T>
where
    T: Serialize,
{
    pub fn ok(data: T, request_id: String) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
            request_id,
        }
    }

    pub fn err(code: &'static str, message: impl Into<String>, request_id: String) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(AgentErrorBody {
                code,
                message: message.into(),
            }),
            request_id,
        }
    }
}

pub fn request_id_or_generate(request_id: Option<RequestId>) -> String {
    request_id
        .map(|request_id| request_id.0)
        .unwrap_or_else(generate_request_id)
}

pub fn ok_response<T>(status: StatusCode, request_id: Option<RequestId>, data: T) -> Response
where
    T: Serialize,
{
    let request_id = request_id_or_generate(request_id);
    (status, Json(AgentEnvelope::ok(data, request_id))).into_response()
}

pub fn err_response<T>(
    status: StatusCode,
    request_id: Option<RequestId>,
    code: &'static str,
    message: impl Into<String>,
) -> Response
where
    T: Serialize,
{
    let request_id = request_id_or_generate(request_id);
    (
        status,
        Json(AgentEnvelope::<T>::err(code, message, request_id)),
    )
        .into_response()
}
