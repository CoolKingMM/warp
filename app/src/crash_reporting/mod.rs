//! Stub implementation of crash reporting with all telemetry and Sentry backdoors permanently removed.

use std::borrow::Cow;
use std::path::Path;
use warpui::AppContext;
use warpui::rendering::GPUDeviceInfo;

pub fn init(_ctx: &mut AppContext) -> bool {
    false
}

pub fn is_initialized() -> bool {
    false
}

pub fn uninit_sentry() {}
pub fn init_cocoa_sentry() {}
pub fn uninit_cocoa_sentry() {}

pub fn set_tag<'a, 'b>(_key: impl Into<Cow<'a, str>>, _value: impl Into<Cow<'b, str>>) {}
pub fn set_client_type_tag(_client_type: &'static str) {}
pub fn set_user_id(
    _user_id: Option<crate::auth::UserUid>,
    _email: Option<String>,
    _ctx: &mut AppContext,
) {
}
pub fn set_gpu_device_info(_info: GPUDeviceInfo) {}
pub fn crash() {
    log::warn!("Manual crash requested, but crash reporting is removed.");
}

pub fn run_minidump_server(_socket_path: &Path) -> anyhow::Result<()> {
    Ok(())
}
