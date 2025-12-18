use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Video {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub what: String,
    pub url: String,
}


impl Video {
    pub fn of(video: Video) -> Video {
        Video {
            id: video.id,
            name: video.name,
            path: video.path,
            what: video.what,
            url: video.url,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VideoRequest {
    pub name: String,
    pub path: String,
}
