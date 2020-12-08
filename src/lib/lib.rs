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
        ProjectsRequest::new().call::<Self::ConfigType>().await
    }

    async fn get_project<T>(project: T) -> Result<ProjectResponse> where T: Into<String> + Send {
        ProjectRequest::new(project).call::<Self::ConfigType>().await
    }

    async fn get_group_info<T>(project: T, group: T) -> Result<ProjectGroupInfoResponse> where T: Into<String> + Send {
        ProjectGroupInfoRequest::new(project, group).call::<Self::ConfigType>().await
    }

    async fn get_group_builds<T>(project: T, group: T) -> Result<ProjectGroupBuildsResponse> where T: Into<String> + Send {
        ProjectGroupBuildsRequest::new(project, group).call::<Self::ConfigType>().await
    }

    async fn get_version_info<T>(project: T, version: T) -> Result<ProjectVersionInfoResponse> where T: Into<String> + Send {
        ProjectVersionInfoRequest::new(project, version).call::<Self::ConfigType>().await
    }

    async fn get_version_builds<T>(project: T, version: T, build: i32) -> Result<ProjectVersionBuildsResponse> where T: Into<String> + Send {
        ProjectVersionBuildsRequest::new(project, version, build).call::<Self::ConfigType>().await
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

pub async fn call_request<T, S>(request: &S) -> Result<S::Response>
    where
        T: PaperClientConfig,
        S: Request + Send + Sync,
        S::Response: DeserializeOwned,
{
    let reader = get_reader::<T>(request.build_request_url()).await?;
    let value = serde_json::from_reader(reader)?;

    Ok(value)
}

#[cfg(test)]
mod test;