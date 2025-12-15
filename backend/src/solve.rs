use walkdir::WalkDir;
use std::{ffi::OsStr, path::Path};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};
use std::fs;

use common::*;


pub async fn remove_video(myvideo: String) -> Result<impl Reply, warp::Rejection> {
    println!("Preparing removal of file {}", myvideo);

    for entry in WalkDir::new("/opt/vids/raw").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let myfn = entry.path().file_name().unwrap();
            if myfn == myvideo.as_str() {
                println!("THis is the file I want to remove: {:?}", entry);

                fs::remove_file(entry.path());
            }
        }
    }

    Ok(StatusCode::OK)
}

pub async fn archive_video(myvideo: String) -> Result<impl Reply, warp::Rejection> {
    println!("Preparing archiving of file {}", myvideo);

    for entry in WalkDir::new("/opt/vids/raw/").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let myfn = entry.path().file_name().unwrap();
            if myfn == myvideo.as_str() {
                let dest = format!("/opt/vids/backup/{}", myvideo);
                println!("This is the file I want to remove: {:?} > to > {:?}", entry, dest);

                fs::rename(entry.path(), dest);
            }
        }
    }

    Ok(StatusCode::OK)
}

pub async fn return_list_video() -> Result<impl Reply, warp::Rejection> {
  println!("Fetching list of available videos");

  let mut count = 0;
  //let myPath: WalkDir::new("/home/rust/vids/");
  let mut videos: Vec<Video> = vec![];
  for entry in WalkDir::new("/opt/vids/raw/").into_iter().filter_map(|e| e.ok()) {
      if entry.file_type().is_file() {
          count +=1;
          let path = entry.path();
          if let Some(extension) = path.extension().and_then(OsStr::to_str) {
              match extension {
                  "mp4" =>
                      videos.push(
                          Video {
                              id: count, name: path.file_name().unwrap().to_str().expect("plop").to_string(),
                              path: path.display().to_string(),
                              what: "video".to_string(),
                              url: path.display().to_string().replace("/opt/vids", "http://bors.greece.local/vids")
                          }),
                  "png" =>
                      videos.push(
                          Video {
                              id: count, name: path.file_name().unwrap().to_str().expect("plop").to_string(),
                              path: path.display().to_string(),
                              what: "picture".to_string(),
                              url: path.display().to_string().replace("/opt/vids", "http://bors.greece.local/vids")
                          }),

                  "gif" =>
                      videos.push(
                          Video {
                              id: count, name: path.file_name().unwrap().to_str().expect("plop").to_string(),
                              path: path.display().to_string(),
                              what: "animation".to_string(),
                              url: path.display().to_string().replace("/opt/vids", "http://bors.greece.local/vids")
                          }),
                  _ => (),
              }
          }
      }
  }

    println!("Found {} files", count);
//    println!("{:?}", videos);
    Ok(json::<Vec<_>>(
        &videos.into_iter().map(Video::of).collect(),
    ))

}
pub async fn handle_rejection(value: String) -> Result<impl Reply, warp::Rejection> {
    println!("Rejecting this {}", value);
    Ok(warp::reply::with_status("OK", StatusCode::OK))
}
