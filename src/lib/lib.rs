pub mod paper;

extern crate hyper;

use hyper::Client;
use paper::*;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use bytes::buf::BufExt;
use bytes::Buf;
use bytes::buf::ext::Reader;

const BASE_URL: &str = "https://papermc.io/api";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn build_url(path: &String) -> String {
    let mut final_url = String::from(BASE_URL);
    final_url.push_str(path.as_str());

    final_url
}

fn build_client() -> Client<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new();
    Client::builder().build::<_, hyper::Body>(https)
}

async fn get(path: String) -> Result<Reader<impl Buf>> {
    let client = build_client();
    let uri = build_url(&path).parse()?;
    println!("GETTING {}", uri);
    let client_response = client.get(uri).await?;
    println!("Response: {}", client_response.status());
    let buf = hyper::body::aggregate(client_response).await?;
    let bytes = buf.reader();
    Ok(bytes)
}

pub async fn get_projects() -> Result<ProjectsResponse> {
    get(String::from(PROJECTS_REQUEST_URL)).await.map(|reader| {
        serde_json::from_reader(reader).expect("Failed to parse json")
    })
}

pub async fn get_project(request: ProjectRequest) -> Result<ProjectResponse> {
    get(project_request_url(request)).await.map(|reader| {
        serde_json::from_reader(reader).expect("Failed to parse json")
    })
}

pub async fn get_group_info(request: ProjectGroupInfoRequest) -> Result<ProjectGroupInfoResponse> {
    get(project_group_info_request_url(request)).await.map(|reader| {
        serde_json::from_reader(reader).expect("Failed to parse json")
    })
}

pub async fn get_group_builds(request: ProjectGroupBuildsRequest) -> Result<ProjectGroupBuildsResponse> {
    get(project_group_builds_request_url(request)).await.map(|reader| {
        serde_json::from_reader(reader).expect("Failed to parse json")
    })
}

pub async fn get_version_info(request: ProjectVersionInfoRequest) -> Result<ProjectVersionInfoResponse> {
    get(project_version_info_request_url(request)).await.map(|reader| {
        serde_json::from_reader(reader).expect("Failed to parse json")
    })
}

pub async fn get_version_builds(request: ProjectVersionBuildsRequest) -> Result<ProjectVersionBuildsResponse> {
    get(project_version_builds_request_url(request)).await.map(|reader| {
        serde_json::from_reader(reader).expect("Failed to parse json")
    })
}

#[cfg(test)]
mod test;