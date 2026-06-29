use std::collections::HashMap;
use std::path::PathBuf;

use chrono::{Duration, Utc};

use super::ProjectManagementModel;
use crate::persistence::model::Project;

fn project(path: PathBuf, added_offset_seconds: i64, pinned: bool) -> Project {
    let now = Utc::now().naive_utc();
    let added_ts = now + Duration::seconds(added_offset_seconds);
    Project {
        path: path.to_string_lossy().to_string(),
        added_ts,
        last_opened_ts: Some(added_ts),
        pinned,
        pinned_ts: pinned.then_some(added_ts),
    }
}

fn model(projects: Vec<Project>) -> ProjectManagementModel {
    ProjectManagementModel {
        projects: projects
            .into_iter()
            .map(|project| (PathBuf::from(&project.path), project))
            .collect::<HashMap<_, _>>(),
        model_event_sender: None,
    }
}

#[test]
fn startup_project_paths_returns_all_pinned_and_latest_unpinned() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let pinned_older = tempdir.path().join("pinned-older");
    let pinned_newer = tempdir.path().join("pinned-newer");
    let unpinned_older = tempdir.path().join("unpinned-older");
    let unpinned_newer = tempdir.path().join("unpinned-newer");
    for path in [
        &pinned_older,
        &pinned_newer,
        &unpinned_older,
        &unpinned_newer,
    ] {
        std::fs::create_dir(path).expect("project directory should be created");
    }

    let model = model(vec![
        project(pinned_newer.clone(), 20, true),
        project(unpinned_older, 30, false),
        project(pinned_older.clone(), 10, true),
        project(unpinned_newer.clone(), 40, false),
    ]);

    assert_eq!(
        model.startup_project_paths(),
        vec![pinned_older, pinned_newer, unpinned_newer]
    );
}

#[test]
fn startup_project_paths_skips_missing_projects() {
    let tempdir = tempfile::tempdir().expect("tempdir should be created");
    let pinned_existing = tempdir.path().join("pinned-existing");
    let pinned_missing = tempdir.path().join("pinned-missing");
    let unpinned_existing = tempdir.path().join("unpinned-existing");
    let unpinned_missing = tempdir.path().join("unpinned-missing");
    std::fs::create_dir(&pinned_existing).expect("project directory should be created");
    std::fs::create_dir(&unpinned_existing).expect("project directory should be created");

    let model = model(vec![
        project(pinned_missing, 10, true),
        project(pinned_existing.clone(), 20, true),
        project(unpinned_existing.clone(), 30, false),
        project(unpinned_missing, 40, false),
    ]);

    assert_eq!(
        model.startup_project_paths(),
        vec![pinned_existing, unpinned_existing]
    );
}
