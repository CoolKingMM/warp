mod collector;
mod context;
pub mod context_provider;
mod events;
mod macros;
pub mod rudder_message;
pub mod secret_redaction;

use std::path::{Path, PathBuf};
use anyhow::Result;

pub use collector::*;
pub use context::telemetry_context;
pub use events::*;

use crate::auth::UserUid;
use crate::settings::PrivacySettingsSnapshot;

/// Filename for file where telemetry events are written on app quit.
const RUDDER_TELEMETRY_EVENTS_FILE_NAME: &str = "rudder_telemetry_events.json";

/// Filepath where the Rudder events should be written on app quit.
fn rudder_event_file_path() -> PathBuf {
    warp_core::paths::secure_state_dir()
        .unwrap_or_else(warp_core::paths::state_dir)
        .join(RUDDER_TELEMETRY_EVENTS_FILE_NAME)
}

/// Removes all telemetry events from the app telemetry event queue.
pub fn clear_event_queue() {
    let _ = warpui::telemetry::flush_events();
}

/// Stub telemetry API with all network transmission and tracking mechanisms physically removed.
pub struct TelemetryApi {
    pub(super) client: http_client::Client,
}

impl Default for TelemetryApi {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryApi {
    pub fn new() -> Self {
        Self {
            client: http_client::Client::default(),
        }
    }

    pub fn flush_and_persist_events(
        &self,
        _max_event_count: usize,
        _settings_snapshot: PrivacySettingsSnapshot,
    ) -> Result<()> {
        Ok(())
    }

    /// Clears any queued events without sending anything over the network.
    pub async fn flush_events(&self, _settings_snapshot: PrivacySettingsSnapshot) -> Result<usize> {
        let _ = warpui::telemetry::flush_events();
        Ok(0)
    }

    /// Cleans up any persisted telemetry file without uploading to Rudderstack.
    pub async fn flush_persisted_events_to_rudder(
        &self,
        path: &Path,
        _settings_snapshot: PrivacySettingsSnapshot,
    ) -> Result<()> {
        if path.exists() {
            let _ = std::fs::remove_file(path);
        }
        Ok(())
    }

    /// Drops any outgoing telemetry event immediately.
    pub async fn send_telemetry_event(
        &self,
        _user_id: Option<UserUid>,
        _anonymous_id: String,
        _event: impl warp_core::telemetry::TelemetryEvent,
        _settings_snapshot: PrivacySettingsSnapshot,
    ) -> Result<()> {
        Ok(())
    }
}
