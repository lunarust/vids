extern crate lazy_static;
//use common::*;
//use walkdir::WalkDir;
//use std::{ffi::OsStr, path::Path};
use warp::{http::{Method},Filter};
//use warp::log;
mod solve;
mod error;


#[tokio::main]
async fn main() {

    println!("Good day ▼(´ᴥ`)▼ ");

    let list = warp::path!("list")
        .and_then(solve::return_list_video);

    let fetch = warp::path!("fetch" / String)
        .and_then(solve::fetch_from_phone);

    let clean = warp::path!("clean" / String)
        .and_then(solve::clean_phone);

    let remove = warp::path!("remove")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(solve::remove_video);

    let archive = warp::path!("archive")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(solve::archive_video);

    let soundout = warp::path!("soundout");
    let soundout_route = soundout
        .and(warp::post())
        .and(warp::body::json())
        .and_then(solve::remove_sound);

    let extractsound = warp::path!("extractsound");
    let extractsound_route = extractsound
        .and(warp::post())
        .and(warp::body::json())
        .and_then(solve::extract_sound);

    let togif = warp::path!("togif");
    let togif_route = togif
        .and(warp::post())
        .and(warp::body::json())
        .and_then(solve::to_gif);

    let routes = list
    .or(soundout_route)
    .or(extractsound_route)
    .or(togif_route)
    .or(remove)
    .or(archive)
    .or(fetch)
    .or(clean)
    .recover(error::handle_rejection)
    .with(
        warp::cors()
            .allow_origin("http://localhost")
            .allow_origin("http://aetes")
            .allow_methods(&[
                Method::OPTIONS,
                Method::GET,
                Method::POST,
                Method::DELETE,
            ])
            .allow_headers(vec!["allow_origin", "allow_any_origin", "Access-Control-Allow-Origin",
                "Referer", "Control-Request-Headers", "Content-Type"])
            .max_age(300)
            .allow_any_origin(),
    );
    warp::serve(routes).run(([0, 0, 0, 0], 9000)).await;
}
