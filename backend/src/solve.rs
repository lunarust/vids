use walkdir::WalkDir;
use std::{ffi::OsStr};
use warp::{http::StatusCode, reply::json, Reply};
use std::fs;
use std::process::Command;
//use std::fmt::format;

use common::*;

pub async fn extract_sound(body: VideoRequest) -> Result<impl Reply, warp::Rejection> {
        println!("Extract sound: {:?}", body);
        let cmd = format!("ffmpeg -i {} {}", body.path, body.path.replace(".mp4", ".mp3"));
        let _ = runit(cmd).await;
        Ok(StatusCode::OK)
}

pub async fn remove_sound(body: VideoRequest) -> Result<impl Reply, warp::Rejection> {
        println!("Remove sound: {:?}", body);
        let cmd = format!("ffmpeg -i {} -c copy -an {}", body.path, body.path.replace(".mp4", ".mp3"));
        let _ = runit(cmd).await;
        Ok(StatusCode::OK)
}
pub async fn to_gif(body: VideoRequest) -> Result<impl Reply, warp::Rejection> {
    println!("Create gif: {:?}", body);
    let cmd = format!(r#"ffmpeg -i {} -vf "select='gt(trunc(t/2),trunc(prev_t/2))',setpts='PTS*0.1',scale=trunc(oh*a/2)*2:320:force_original_aspect_ratio=decrease,pad=trunc(oh*a/2)*2:320:-1:-1" -loop 0 -an {}"#,
        body.path, body.path.replace(".mp4", ".gif")
    );

    let _ = runit(cmd).await;
    Ok(StatusCode::OK)
}
pub async fn fetch_from_phone(phone: String) -> Result<impl Reply, warp::Rejection> {
    println!("Fetch videos: {:?}", phone);
    let cmd = format!("cd /opt/scripts/ && ./fetch_videos.sh true {}", phone);

    let _ = runit(cmd).await;



    Ok(StatusCode::OK)
}
pub async fn clean_phone(phone: String) -> Result<impl Reply, warp::Rejection> {
    println!("Fetch videos: {:?}", phone);
    let cmd = format!("cd /opt/scripts/ && ./fetch_videos.sh true {}", phone);

    let _ = runit(cmd).await;
    Ok(StatusCode::OK)
}

async fn runit(cmd: String) -> Result<(), std::io::Error> {
    println!("Running command: {:?}", cmd);
        let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap()
        //.expect("failed to execute process")
        ;
    let stdout = String::from_utf8(output.stdout).unwrap();

    println!("{}", stdout);
    Ok(())
}

pub async fn remove_video(body: VideoRequest) -> Result<impl Reply, warp::Rejection> {
    println!("Preparing removal of file {}", body.name);

    let _ = fs::remove_file(body.path);

    Ok(StatusCode::OK)
}

pub async fn archive_video(body: VideoRequest) -> Result<impl Reply, warp::Rejection> {
    println!("Preparing archiving of file {}", body.name);
    let archive_path = format!("/opt/vids/backup/{}", body.name);
    let _ = fs::rename(body.path, archive_path);


    Ok(StatusCode::OK)
}

pub async fn return_list_video(dir: String) -> Result<impl Reply, warp::Rejection> {
  println!("Fetching list of available videos");

  let mut count = 0;
  //let myPath: WalkDir::new("/home/rust/vids/");
  let mut videos: Vec<Video> = vec![];
  for entry in WalkDir::new(format!("/opt/vids/{}/", dir)).into_iter().filter_map(|e| e.ok()) {
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
