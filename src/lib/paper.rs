use serde::{Serialize, Deserialize};

use super::call_request;
use super::{Result, PaperClientConfig};

pub trait Request {
    type Response;

    fn build_request_url(&self) -> String;
}

macro_rules! paper_struct {
    ($i:ident $($value:ident => $t:ty),+ $(,)?) => (
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $i { $(pub $value: $t,)+ }
    );
    ($i:ident $($value:ident => $t:ty = $ext:ty),+ $(,)?) => (
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $i { $(pub $value: $t,)+ }
    );
    ($i:ident | $url:expr, $resp:ty) => (
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $i;
        impl $i {
            pub fn new() -> Self { Self {} }
            pub async fn call<T>(&self) -> Result<$resp> where T: PaperClientConfig { call_request::<T, Self>(self).await }
        }
        impl Request for $i {
            type Response = $resp;
            fn build_request_url(&self) -> String {$url.into()}
        }
    );
    ($i:ident $($value:ident => $t:ty),+$(,)? | $url:expr, $resp:ty) => (
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $i { $(pub $value: $t,)+ }

        impl $i {
            pub fn new<T>($($value: T),+ ) -> Self where T: Into<String> { Self { $($value: $value.into()),+ } }
            pub async fn call<T>(&self) -> Result<$resp> where T: PaperClientConfig { call_request::<T, Self>(self).await }
        }

        impl Request for $i {
            type Response = $resp;
            fn build_request_url(&self) -> String { format!($url, $(self.$value),+) }
        }
    );
    ($i:ident $($value:ident => $t:ty = $ext:ty),+$(,)? | $url:expr, $resp:ty) => (
        #[derive(Serialize, Deserialize, Debug)]
        pub struct $i { $(pub $value: $t,)+ }

        impl $i {
            pub fn new<T>($($value: $ext),+ ) -> Self where T: Into<String> { Self { $($value: $value.into()),* } }
            pub async fn call<T>(&self) -> Result<$resp> where T: PaperClientConfig { call_request::<T, Self>(self).await }
        }

        impl Request for $i {
            type Response = $resp;
            fn build_request_url(&self) -> String { format!($url, $(self.$value),+) }
        }
    );
}

paper_struct!(ChangesInfo commit => String, summary => String, message => String);
paper_struct!(ApplicationInfo name => String, sha256 => String);
paper_struct!(DownloadInfo application => ApplicationInfo = ApplicationInfo);

paper_struct! { BuildInfo
    build => i32 = i32,
    time => String = T,
    version => String = T,
    changes => Vec<ChangesInfo> = Vec<ChangesInfo>,
    downloads => DownloadInfo = DownloadInfo,
}

paper_struct!(ProjectsResponse projects => Vec<String> = Vec<String>);

paper_struct!(ProjectsRequest | "/v2/projects", ProjectsResponse);

paper_struct! { ProjectResponse
    project_id => String = T,
    project_name => String = T,
    version_groups => Vec<String> = Vec<String>,
    versions => Vec<String> = Vec<String>
}

paper_struct!(ProjectRequest project => String | "/v2/projects/{}", ProjectResponse);

paper_struct! { ProjectGroupInfoResponse
    project_id => String = T,
    project_name => String = T,
    version_group => String = T,
    versions => Vec<String> = Vec<String>,
}

paper_struct!(ProjectGroupInfoRequest project => String, version_group => String | "/v2/projects/{}/version_group/{}", ProjectGroupInfoResponse);

paper_struct! { ProjectGroupBuildsResponse
    project_id => String = T,
    project_name => String = T,
    version_group => String = T,
    version => Vec<String> = Vec<String>,
    builds => Vec<BuildInfo> = Vec<BuildInfo>,
}

paper_struct! { ProjectGroupBuildsRequest
    project => String,
    version_group => String,
    | "/v2/projects/{}/version_group/{}/builds", ProjectGroupBuildsResponse
}

paper_struct! { ProjectVersionInfoResponse
    project_id => String = T,
    project_name => String = T,
    version => String = T,
    builds => Vec<i32> = Vec<i32>,
}

paper_struct!(ProjectVersionInfoRequest project => String, version => String | "/v2/projects/{}/versions/{}", ProjectVersionInfoResponse);

paper_struct! { ProjectVersionBuildsResponse
    project_id => String = T,
    project_name => String = T,
    version => String = T,
    build => i32 = i32,
    time => String = T,
    changes => Vec<ChangesInfo> = Vec<ChangesInfo>,
    downloads => DownloadInfo = DownloadInfo,
}

paper_struct! { ProjectVersionBuildsRequest
    project => String = T,
    version => String = T,
    build => i32 = i32,
    | "/v2/projects/{}/versions/{}/builds/{}", ProjectVersionBuildsResponse
}