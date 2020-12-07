pub mod paper;

extern crate hyper;

use async_trait::async_trait;
use hyper::{Client, StatusCode, Body, Response};
use hyper::body::HttpBody;
use paper::*;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use bytes::buf::BufExt;
use bytes::Buf;
use bytes::buf::ext::Reader;
use serde::de::DeserializeOwned;

const BASE_URL: &str = "https://papermc.io/api";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait]
pub trait PaperClientConfig {
    type ConfigType: PaperClientConfig;

    fn debug() -> bool;

    async fn get_projects() -> Result<ProjectsResponse> {
        get_projects::<Self::ConfigType>().await
    }

    async fn get_project<T>(project: T) -> Result<ProjectResponse>
        where T: Into<String> + Send
    {
        get_project::<Self::ConfigType>(ProjectRequest::new(project)).await
    }

    async fn get_group_info<T>(project: T, group: T) -> Result<ProjectGroupInfoResponse>
        where T: Into<String> + Send
    {
        get_group_info::<Self::ConfigType>(ProjectGroupInfoRequest::new(project, group)).await
    }

    async fn get_group_builds<T>(project: T, group: T) -> Result<ProjectGroupBuildsResponse>
        where T: Into<String> + Send
    {
        get_group_builds::<Self::ConfigType>(ProjectGroupBuildsRequest::new(project, group)).await
    }

    async fn get_version_info<T>(project: T, version: T) -> Result<ProjectVersionInfoResponse>
        where T: Into<String> + Send
    {
        get_version_info::<Self::ConfigType>(ProjectVersionInfoRequest::new(project, version)).await
    }

    async fn get_version_builds<T>(project: T, version: T, build: i32) -> Result<ProjectVersionBuildsResponse>
        where T: Into<String> + Send
    {
        get_version_builds::<Self::ConfigType>(ProjectVersionBuildsRequest::new(project, version, build)).await
    }
}

pub struct PaperClientDebug;

impl PaperClientConfig for PaperClientDebug {
    type ConfigType = Self;

    fn debug() -> bool {
        true
    }
}

pub struct PaperClient;

impl PaperClientConfig for PaperClient {
    type ConfigType = Self;

    fn debug() -> bool {
        false
    }
}

fn build_url(path: &String) -> String {
    let mut final_url = String::from(BASE_URL);
    final_url.push_str(path.as_str());
    final_url
}

fn build_client() -> Client<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

async fn get_reader<T>(path: String) -> Result<Reader<impl Buf>>
    where T: PaperClientConfig
{
    let client = build_client();
    let uri = build_url(&path).parse()?;
    if T::debug() {
        println!("GETTING {}", uri);
    }
    let mut client_response: Response<Body> = client.get(uri).await?;
    if T::debug() {
        println!("Response: {}", client_response.status());
    }

    if client_response.status().ne(&StatusCode::from_u16(200)?) {
        let mut error = String::from("");
        while let Some(chunk) = client_response.body_mut().data().await {
            error.push_str(&String::from_utf8_lossy(&chunk?));
        }
        return Err(Box::from(error));
    }

    let buf = hyper::body::aggregate(client_response).await?;
    let bytes = buf.reader();
    Ok(bytes)
}

fn parse_json<'a, T>(buffer: Reader<impl Buf>) -> T
    where T: DeserializeOwned
{
    serde_json::from_reader(buffer).expect("Failed to parse json")
}

async fn get_json<T, K, V>(url: K) -> Result<V>
    where
        T: PaperClientConfig,
        K: Into<String>,
        V: DeserializeOwned
{
    get_reader::<T>(url.into()).await.map(|reader| parse_json(reader))
}

pub async fn get_projects<T>() -> Result<ProjectsResponse>
    where T: PaperClientConfig
{
    get_json::<T, _, _>(PROJECTS_REQUEST_URL).await
}

pub async fn get_project<T>(request: ProjectRequest) -> Result<ProjectResponse>
    where T: PaperClientConfig
{
    get_json::<T, _, _>(project_request_url(request)).await
}

pub async fn get_group_info<T>(request: ProjectGroupInfoRequest) -> Result<ProjectGroupInfoResponse>
    where T: PaperClientConfig
{
    get_json::<T, _, _>(project_group_info_request_url(request)).await
}

pub async fn get_group_builds<T>(request: ProjectGroupBuildsRequest) -> Result<ProjectGroupBuildsResponse>
    where T: PaperClientConfig
{
    get_json::<T, _, _>(project_group_builds_request_url(request)).await
}

pub async fn get_version_info<T>(request: ProjectVersionInfoRequest) -> Result<ProjectVersionInfoResponse>
    where T: PaperClientConfig
{
    get_json::<T, _, _>(project_version_info_request_url(request)).await
}

pub async fn get_version_builds<T>(request: ProjectVersionBuildsRequest) -> Result<ProjectVersionBuildsResponse>
    where T: PaperClientConfig
{
    get_json::<T, _, _>(project_version_builds_request_url(request)).await
}

#[cfg(test)]
mod test;