use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChangesInfo {
    pub commit: String,
    pub summary: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationInfo {
    pub name: String,
    pub sha256: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadInfo {
    pub application: ApplicationInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildInfo {
    pub build: i32,
    pub time: String,
    pub version: String,
    pub changes: Vec<ChangesInfo>,
    pub downloads: DownloadInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectsResponse {
    pub projects: Vec<String>,
}

pub const PROJECTS_REQUEST_URL: &str = "/v2/projects";

pub struct ProjectRequest {
    project: String,
}

impl ProjectRequest {
    pub fn new<T>(project: T) -> Self
        where T: Into<String>
    {
        Self { project: project.into() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectResponse {
    pub project_id: String,
    pub project_name: String,
    pub version_groups: Vec<String>,
    pub versions: Vec<String>,
}

pub fn project_request_url(request: ProjectRequest) -> String {
    format!("/v2/projects/{}", request.project)
}

pub struct ProjectGroupInfoRequest {
    project: String,
    version_group: String,
}

impl ProjectGroupInfoRequest {
    pub fn new<T>(project: T, version_group: T) -> Self
        where T: Into<String>
    {
        Self { project: project.into(), version_group: version_group.into() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectGroupInfoResponse {
    pub project_id: String,
    pub project_name: String,
    pub version_group: String,
    pub versions: Vec<String>,
}

pub fn project_group_info_request_url(request: ProjectGroupInfoRequest) -> String {
    format!("/v2/projects/{}/version_group/{}", request.project, request.version_group)
}

pub struct ProjectGroupBuildsRequest {
    project: String,
    version_group: String,
}

impl ProjectGroupBuildsRequest {
    pub fn new<T>(project: T, version_group: T) -> Self
        where T: Into<String>
    {
        Self { project: project.into(), version_group: version_group.into() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectGroupBuildsResponse {
    pub project_id: String,
    pub project_name: String,
    pub version_group: String,
    pub versions: Vec<String>,
    pub builds: Vec<BuildInfo>,
}

pub fn project_group_builds_request_url(request: ProjectGroupBuildsRequest) -> String {
    format!("/v2/projects/{}/version_group/{}/builds", request.project, request.version_group)
}

pub struct ProjectVersionInfoRequest {
    project: String,
    version: String,
}

impl ProjectVersionInfoRequest {
    pub fn new<T>(project: T, version: T) -> Self
        where T: Into<String>
    {
        Self { project: project.into(), version: version.into() }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectVersionInfoResponse {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<i32>,
}

pub fn project_version_info_request_url(request: ProjectVersionInfoRequest) -> String {
    format!("/v2/projects/{}/versions/{}", request.project, request.version)
}

pub struct ProjectVersionBuildsRequest {
    project: String,
    version: String,
    build: i32,
}

impl ProjectVersionBuildsRequest {
    pub fn new<T>(project: T, version: T, build: i32) -> Self
        where T: Into<String>
    {
        Self { project: project.into(), version: version.into(), build }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectVersionBuildsResponse {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub build: i32,
    pub time: String,
    pub changes: Vec<ChangesInfo>,
    pub downloads: DownloadInfo,
}

pub fn project_version_builds_request_url(request: ProjectVersionBuildsRequest) -> String {
    format!("/v2/projects/{}/versions/{}/builds/{}", request.project, request.version, request.build)
}