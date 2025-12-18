extern crate lazy_static;
//use common::*;
//use walkdir::WalkDir;
//use std::{ffi::OsStr, path::Path};
use warp::{
    http::{header, Method},
    Filter, Rejection,
};
mod solve;

#[tokio::main]
async fn main() {

    println!("Good day ▼(´ᴥ`)▼ ");

    //solve::return_list_video().await;

    let hello = warp::path!("hello" / String) // 3.
        .map(|name| format!("Hello, {}!", name)); // 4.

    let list = warp::path!("list" / String) // 3.
//           .and(warp::body::json())
           .and_then(solve::return_list_video);

    let remove = warp::path!("remove" / String)
        .and_then(solve::remove_video);

    let archive = warp::path!("archive" / String)
        .and_then(solve::archive_video);

    let soundout = warp::path!("soundout")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(solve::remove_video);

    let route = warp::method()
        .map(|method| {
            format!("You sent a {} request!", method)
    });


    let routes = list
    .or(remove)
    .or(archive)
    .or(hello)
    .or(soundout)
    .with(
        warp::cors()
            .allow_origin("http://localhost")
            .allow_origin("http://aetes")
            .allow_methods(&[
                Method::OPTIONS,
                Method::GET,
                Method::POST,
                Method::DELETE,
                Method::PUT,
                Method::HEAD,
                Method::PATCH,
                Method::DELETE,
            ])
            .allow_headers(vec!["allow_origin", "allow_any_origin", "Access-Control-Allow-Origin",
                "Referer", "Control-Request-Headers", "Content-Type"])
            .max_age(300)
            .allow_any_origin(),
    );
    warp::serve(routes).run(([0, 0, 0, 0], 9000)).await;
}
