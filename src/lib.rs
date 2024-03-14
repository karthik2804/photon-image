use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

extern crate photon_rs;
use photon_rs::filters;
use photon_rs::native::{open_image, save_image};

/// A simple Spin HTTP component.
#[http_component]
fn handle_photon_image(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let mut img = open_image("image.jpg").expect("File should open");
    filters::filter(&mut img, "twenties");

    // Write the new image to the filesystem.
    save_image(img, "new_image.jpg").expect("File should be saved");

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}
