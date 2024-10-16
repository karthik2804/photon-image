use photon_rs::transform::resize;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

extern crate photon_rs;
extern crate url;
use std::str::FromStr;
use url::Url;

use photon_rs::native::open_image;

/// A simple Spin HTTP component.
#[http_component]
fn handle_photon_image(req: Request) -> anyhow::Result<impl IntoResponse> {
    let parsed_url = Url::parse("http://fake.com").unwrap().join(req.uri())?;

    // Extract query parameters using `url::Url`
    let width = parsed_url
        .query_pairs()
        .find(|(key, _)| key == "width")
        .and_then(|(_, value)| u32::from_str(&value).ok());
    let height = parsed_url
        .query_pairs()
        .find(|(key, _)| key == "height")
        .and_then(|(_, value)| u32::from_str(&value).ok());

    let mut img = open_image("image.jpg").expect("File should open");

    if let (Some(w), Some(h)) = (width, height) {
        img = resize(&img, w, h, photon_rs::transform::SamplingFilter::Gaussian);
    }

    let bytes = img.get_bytes_jpeg(75);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(bytes)
        .build())
}
