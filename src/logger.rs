use chrono;
use log::{error, info};
use serde_json::{json, Value};

pub struct Logger {
    context: Value,
}

impl Logger {
    pub fn new(context: Value) -> Self {
        Self { context }
    }

    pub fn info(&self, message: &str, additional_context: Option<Value>) {
        let log_entry = self.create_log_entry("INFO", message, additional_context);
        info!("{}", log_entry);
    }

    pub fn error(&self, message: &str, additional_context: Option<Value>) {
        let log_entry = self.create_log_entry("ERROR", message, additional_context);
        error!("{}", log_entry);
    }

    fn create_log_entry(
        &self,
        level: &str,
        message: &str,
        additional_context: Option<Value>,
    ) -> String {
        let mut entry = json!({
            "level": level,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "context": self.context,
        });

        if let Some(additional) = additional_context {
            entry["additional_context"] = additional;
        }

        serde_json::to_string(&entry).unwrap()
    }
}
