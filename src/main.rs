extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate time;
extern crate urlencoding;
#[macro_use]
extern crate log;
#[macro_use]
extern crate askama;
extern crate hyper;
extern crate mime;

mod logger;
mod middleware;

use iron::prelude::*;
use iron::status;
use iron::request::Request;
use hyper::header::ContentType;
use router::Router;
use staticfile::Static;
use mount::Mount;
use askama::Template;
use std::env;
use std::path::Path;

#[derive(Template)]
#[template(path = "meme.svg")]
struct MemeTemplate<'a> {
    image_path: &'a str,
    top_text: &'a str,
    bottom_text: &'a str,
}

fn favicon(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::NotFound, "nothing")))
}

fn generate(req: &mut Request) -> IronResult<Response> {
    let router = req.extensions.get::<Router>().unwrap();
    let memename = format!("{}.jpg", router.find("memename").unwrap());

    let first_line = urlencoding::decode(router.find("first_line").unwrap()).unwrap();
    let second_line = urlencoding::decode(router.find("second_line").unwrap()).unwrap();

    let templ = MemeTemplate {
        image_path: memename.as_str(),
        top_text: first_line.as_str(),
        bottom_text: second_line.as_str()
    };

    let mut response = Response::with((status::Ok, templ.render()));
    let svg_type: mime::Mime = "image/svg+xml;charset=utf-8".parse().unwrap();
    response.headers.set(ContentType(svg_type));
    Ok(response)
}

fn main() {
    let server_address: String;
    match env::var("MEMEGEN_SERVER") {
        Ok(s) => server_address = s,
        _ => server_address = "localhost:3000".to_string(),
    };
    logger::init_info_log().unwrap();

    let mut router = Router::new();
    router.get("/favicon.ico", favicon, "favicon");
    router.get("/gen/:memename/:first_line/:second_line", generate, "generate");

    let mut mount = Mount::new();
    mount.mount("/images/", Static::new(Path::new("images/")));
    mount.mount("/", router);

    let mut chain = Chain::new(mount);
    chain.link_before(middleware::ResponseTime);
    chain.link_after(middleware::ResponseTime);
    info!("Starting server at {}", server_address);
    Iron::new(chain).http(server_address.as_str()).unwrap();
}
