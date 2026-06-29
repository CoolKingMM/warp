use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::SyncSender;

use chrono::Utc;
use warpui::{Entity, ModelContext, SingletonEntity};

use crate::persistence::model::Project;
use crate::persistence::ModelEvent;

#[derive(Debug)]
pub enum ProjectEvent {
    Added {
        #[expect(unused, reason = "TODO(jparker): #pod-code-mode wip")]
        path: PathBuf,
    },
    #[expect(unused, reason = "TODO(jparker): #pod-code-mode wip")]
    Removed { path: PathBuf },
    #[expect(unused, reason = "TODO(jparker): #pod-code-mode wip")]
    Updated { path: PathBuf },
}

pub struct ProjectManagementModel {
    projects: HashMap<PathBuf, Project>,
    model_event_sender: Option<SyncSender<ModelEvent>>,
}

impl Entity for ProjectManagementModel {
    type Event = ProjectEvent;
}

impl SingletonEntity for ProjectManagementModel {}

impl ProjectManagementModel {
    /// Create a new Projects model with persisted data
    pub fn new(
        persisted_projects: Vec<Project>,
        model_event_sender: Option<SyncSender<ModelEvent>>,
        _ctx: &mut ModelContext<Self>,
    ) -> Self {
        log::debug!("Loading {} persisted projects", persisted_projects.len());

        let projects = persisted_projects
            .into_iter()
            .map(|project| (PathBuf::from(&project.path), project))
            .collect();

        Self {
            projects,
            model_event_sender,
        }
    }

    /// Add a project to the list. If it already exists, update the last_opened_ts.
    pub fn upsert_project(&mut self, path: PathBuf, ctx: &mut ModelContext<Self>) {
        let now = Utc::now().naive_utc();

        let project = if let Some(existing_project) = self.projects.get_mut(&path) {
            // Update existing project's last opened time
            existing_project.last_opened_ts = Some(now);
            existing_project.clone()
        } else {
            // Create new project
            let project = Project {
                path: path.to_string_lossy().to_string(),
                added_ts: now,
                last_opened_ts: Some(now),
                pinned: false,
                pinned_ts: None,
            };
            self.projects.insert(path.clone(), project.clone());
            project
        };
        self.save_project(project);
        ctx.emit(ProjectEvent::Added { path });
    }

    pub fn is_project_pinned(&self, path: &PathBuf) -> bool {
        self.projects
            .get(path)
            .is_some_and(|project| project.pinned)
    }

    pub fn set_project_pinned(
        &mut self,
        path: PathBuf,
        pinned: bool,
        ctx: &mut ModelContext<Self>,
    ) {
        let now = Utc::now().naive_utc();

        let project = if let Some(existing_project) = self.projects.get_mut(&path) {
            existing_project.pinned = pinned;
            existing_project.pinned_ts = pinned.then_some(now);
            existing_project.clone()
        } else {
            let project = Project {
                path: path.to_string_lossy().to_string(),
                added_ts: now,
                last_opened_ts: Some(now),
                pinned,
                pinned_ts: pinned.then_some(now),
            };
            self.projects.insert(path.clone(), project.clone());
            project
        };

        self.save_project(project);
        ctx.emit(ProjectEvent::Updated { path });
    }

    pub fn toggle_project_pinned(&mut self, path: PathBuf, ctx: &mut ModelContext<Self>) {
        let pinned = !self.is_project_pinned(&path);
        self.set_project_pinned(path, pinned, ctx);
    }

    pub fn startup_project_paths(&self) -> Vec<PathBuf> {
        let mut pinned_projects = self
            .projects
            .values()
            .filter(|project| project.pinned)
            .collect::<Vec<_>>();
        pinned_projects
            .sort_by_key(|project| project.pinned_ts.unwrap_or_else(|| project.last_used_at()));

        let mut paths = pinned_projects
            .into_iter()
            .map(|project| PathBuf::from(&project.path))
            .filter(|path| path.is_dir())
            .collect::<Vec<_>>();

        let latest_unpinned = self
            .projects
            .values()
            .filter(|project| !project.pinned)
            .map(|project| (project.last_used_at(), PathBuf::from(&project.path)))
            .filter(|(_, path)| path.is_dir())
            .max_by_key(|(last_used_at, _)| *last_used_at)
            .map(|(_, path)| path);

        if let Some(path) = latest_unpinned {
            if !paths.contains(&path) {
                paths.push(path);
            }
        }

        paths
    }

    pub fn all_projects(&self) -> impl Iterator<Item = &Project> {
        self.projects.values()
    }

    /// Save a project to the database
    fn save_project(&self, project: Project) {
        if let Some(sender) = &self.model_event_sender {
            let event = ModelEvent::UpsertProject { project };
            if let Err(err) = sender.send(event) {
                log::error!("Failed to save project to database: {err}");
            }
        }
    }
}

#[cfg(test)]
#[path = "projects_tests.rs"]
mod tests;
