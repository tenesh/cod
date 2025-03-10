use serde::Serialize;

pub const APP_SETUP_EVENT_ID: &str = r#"app_setup_event"#;

#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AppSetupEventStatus {
    Loading,
    Failed,
    Success,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSetupEvent {
    status: AppSetupEventStatus,
    message: String,
}

impl AppSetupEvent {
    pub fn new(status: AppSetupEventStatus, message: String) -> Self {
        Self { status, message }
    }

    pub fn fail(message: String) -> Self {
        Self::new(AppSetupEventStatus::Failed, message)
    }

    pub fn success(message: String) -> Self {
        Self::new(AppSetupEventStatus::Success, message)
    }

    pub fn loading(message: String) -> Self {
        Self::new(AppSetupEventStatus::Loading, message)
    }
}
