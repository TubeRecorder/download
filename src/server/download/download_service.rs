use log::info;
use std::process::Command;
use tonic::{
    Request,
    Response,
    Status,
};

use crate::download_api::{
    download_server::Download,
    DownloadRequest,
    DownloadResponse,
};

#[derive(Default)]
pub struct DownloadService {}

#[tonic::async_trait]
impl Download for DownloadService {
    async fn download_video(
        &self,
        request: Request<DownloadRequest>,
    ) -> Result<Response<DownloadResponse>, Status> {
        info!("{:?}", request);

        let req = request.get_ref();

        if req.url.is_empty() {
            return Err(Status::invalid_argument("Empty URL"));
        }

        let local_path = if req.local_path.is_empty() {
            String::from("./")
        }
        else {
            req.local_path.clone()
        };

        let _output = match Command::new("mkdir")
            .arg("-p")
            .arg(local_path.clone())
            .output()
        {
            Ok(x) => x,
            Err(e) => {
                return Err(Status::failed_precondition(format!(
                    "Directory creation failed at \"{}\": {}",
                    local_path.clone(),
                    e.to_string()
                )))
            },
        };

        let output = match Command::new("yt-dlp")
            .arg("-P")
            .arg(local_path.clone())
            .arg(req.url.clone())
            .output()
        {
            Ok(x) => x,
            Err(e) => {
                return Err(Status::aborted(format!(
                    "Download failed for \"{}\": {}",
                    req.url.clone(),
                    e.to_string()
                )))
            },
        };

        Ok(Response::new(DownloadResponse {
            status: output.status.code().unwrap_or(1),
        }))
    }
}
