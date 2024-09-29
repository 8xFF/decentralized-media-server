use std::fmt::Display;

use crate::protobuf;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AppContext {
    pub app: Option<String>, //if it is none that mean root app
}

impl Display for AppContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(app) = &self.app {
            f.write_fmt(format_args!("App(\"{app}\")"))
        } else {
            f.write_fmt(format_args!("App(ROOT)"))
        }
    }
}

impl From<protobuf::shared::AppContext> for AppContext {
    fn from(value: protobuf::shared::AppContext) -> Self {
        Self { app: value.app }
    }
}

impl From<Option<protobuf::shared::AppContext>> for AppContext {
    fn from(value: Option<protobuf::shared::AppContext>) -> Self {
        Self { app: value.and_then(|v| v.app) }
    }
}

impl From<AppContext> for protobuf::shared::AppContext {
    fn from(value: AppContext) -> Self {
        Self { app: value.app }
    }
}
