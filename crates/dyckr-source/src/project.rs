mod access_check;
mod discovery_checks;
pub use discovery_checks::DiscoveryChecks;
mod capture_problem;
mod capture_vendor;
mod captured_file;
mod captured_sources;
mod discovered;
mod entry_metadata;
mod handler_node;
mod included_folders;
mod project_discovery;
mod role_source;
mod route_graph;
mod route_node;
mod scheduled_worker;
mod snapshot_context;
mod source_root_step;
mod source_walk;
mod view_kind;
mod view_route;
mod view_source;
mod worker_schedules;

pub use dyckr::companion_kind::CompanionKind;
pub use dyckr::native_companion::NativeCompanion;
pub use handler_node::HandlerNode;
pub use project_discovery::ProjectDiscovery;
pub use route_graph::RouteGraph;
pub use route_node::RouteNode;
pub use scheduled_worker::ScheduledWorker;
pub use view_kind::ViewKind;

use crate::handler_declaration::{DeclaredHandler, OwnedSourceRoot};
use cap_std::fs::Dir;
use capture_problem::CaptureProblem;
use capture_vendor::CaptureVendor;
use captured_file::CapturedFile;
use captured_sources::CapturedSources;
use discovered::Discovered;
use entry_metadata::EntryMetadata;
use included_folders::IncludedFolders;
use role_source::RoleSource;
use snapshot_context::SnapshotContext;
use source_root_step::SourceRootStep;
use source_walk::SourceWalk;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use view_route::ViewRoute;
use view_source::ViewSource;
use worker_schedules::WorkerSchedules;

#[derive(Clone, Debug)]
pub struct Project {
    root: PathBuf,
    route_graph: RouteGraph,
    snapshot: Arc<OwnedSourceRoot>,
    capability: Arc<Dir>,
    middleware: Option<Arc<DeclaredHandler>>,
    identity: Option<Arc<DeclaredHandler>>,
    topics: dyckr::topic_table::TopicTable,
    workers: Vec<ScheduledWorker>,
    mount: dyckr::routing::MountPath,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        (&self.root, &self.route_graph) == (&other.root, &other.route_graph)
    }
}

impl Eq for Project {}

impl Project {
    pub const REMOTE_ROOTS: [&'static str; 2] = ["client/remotes", "client/clients"];

    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    #[must_use]
    pub const fn route_graph(&self) -> &RouteGraph {
        &self.route_graph
    }

    pub fn snapshot_root(&self) -> &Path {
        self.snapshot.path()
    }

    pub fn load_application_for_server(
        &self,
        host: Option<dyckr::safety::ServerHost>,
        port: Option<u16>,
    ) -> Result<crate::application::ApplicationConfig, crate::Failure> {
        crate::application::ApplicationConfig::load_directory_for_server(
            &self.capability(),
            host,
            port,
        )
    }

    pub fn capability(&self) -> Arc<Dir> {
        Arc::clone(&self.capability)
    }

    pub fn middleware(&self) -> Option<&Arc<DeclaredHandler>> {
        self.middleware.as_ref()
    }

    pub fn identity_provider(&self) -> Option<&Arc<DeclaredHandler>> {
        self.identity.as_ref()
    }

    pub fn topics(&self) -> &dyckr::topic_table::TopicTable {
        &self.topics
    }

    pub fn workers(&self) -> &[ScheduledWorker] {
        &self.workers
    }

    pub const fn mount(&self) -> &dyckr::routing::MountPath {
        &self.mount
    }

    pub fn invocation_sources(&self) -> impl Iterator<Item = &Arc<DeclaredHandler>> + '_ {
        self.route_graph
            .routes()
            .iter()
            .flat_map(|route| route.handlers().iter().map(HandlerNode::source))
            .chain(self.middleware.iter())
            .chain(self.identity.iter())
            .chain(self.workers.iter().map(ScheduledWorker::source))
    }
}

pub fn companion_kind(name: &str) -> Option<CompanionKind> {
    CompanionKind::from_file_name(name)
}

#[cfg(test)]
mod tests;
