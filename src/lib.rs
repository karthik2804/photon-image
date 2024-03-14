use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

extern crate photon_rs;
use photon_rs::filters;
use photon_rs::native::open_image;

/// A simple Spin HTTP component.
#[http_component]
fn handle_photon_image(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut img = open_image("image.jpg").expect("File should open");
    filters::filter(&mut img, "oceanic");

    let bytes = img.get_bytes_jpeg(255);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(bytes)
        .build())
}
