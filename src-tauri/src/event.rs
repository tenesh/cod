use serde::Serialize;

pub const EVENT_SETUP_FAILED: &str = r#"setup_failed"#;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetupFailed {
    message: String,
}

impl SetupFailed {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
