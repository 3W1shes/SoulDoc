use crate::{
    error::AppError,
    services::auth::{OptionalUser, User},
};

pub type AgentAuthResult<T> = std::result::Result<T, AppError>;

pub fn require_user(user: Option<User>) -> AgentAuthResult<User> {
    user.ok_or_else(|| AppError::unauthorized("authorization required"))
}

pub fn has_permission(user: &User, permission: &str) -> bool {
    user.permissions.iter().any(|value| value == permission)
        || user.permissions.iter().any(|value| value == "docs.admin")
        || user.roles.iter().any(|value| value == "admin")
}

pub fn require_permission(user: Option<User>, permission: &str) -> AgentAuthResult<User> {
    let user = require_user(user)?;
    if has_permission(&user, permission) {
        Ok(user)
    } else {
        Err(AppError::forbidden(format!(
            "missing permission: {permission}"
        )))
    }
}

pub fn into_optional_user(optional_user: OptionalUser) -> Option<User> {
    optional_user.0
}
