use std::sync::Arc;
use warpui::{Entity, ModelContext, SingletonEntity};
use crate::server::server_api::ServerApi;

/// App singleton for telemetry collection (all background collection and reporting permanently removed).
pub struct TelemetryCollector {
    _server_api: Arc<ServerApi>,
}

impl TelemetryCollector {
    pub fn new(server_api: Arc<ServerApi>) -> Self {
        Self {
            _server_api: server_api,
        }
    }

    pub fn initialize_telemetry_collection(&self, _ctx: &mut ModelContext<TelemetryCollector>) {}

    pub fn write_telemetry_events_to_disk(&self, _ctx: &mut ModelContext<TelemetryCollector>) {}
}

impl Entity for TelemetryCollector {
    type Event = ();
}

impl SingletonEntity for TelemetryCollector {}
