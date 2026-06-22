//! The viewer is a client that joins a shared session.
mod event_loop;
pub(crate) mod history_model;
mod network;
#[cfg(not(feature = "oss_slim"))]
pub(crate) mod orchestration_viewer_model;
pub(crate) mod terminal_manager;
pub(crate) use terminal_manager::TerminalManager;

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
