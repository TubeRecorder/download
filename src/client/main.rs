#[path = "../proto/download-api.rs"]
mod download_api;

use download_api::{
    download_client::DownloadClient,
    DownloadRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static(
        "http://0.0.0.0:50051",
    )
    .connect()
    .await?;

    let mut client = DownloadClient::new(channel);

    let request = tonic::Request::new(DownloadRequest {
        url: String::from(
            "https://www.youtube.com/watch?v=s1HcU7taTbo",
        ),
        local_path: String::from("./temp"),
    });

    // sending request and waiting for response
    let response = client
        .download_video(request)
        .await?
        .into_inner();

    println!("RESPONSE={:?}", response);

    Ok(())
}
