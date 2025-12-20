use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq)]
pub struct Video {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub url: String,
    pub archived: bool,
}


impl Video {
    pub fn of(video: Video) -> Video {
        Video {
            id: video.id,
            name: video.name,
            path: video.path,
            url: video.url,
            archived: video.archived,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VideoRequest {
    pub name: String,
    pub path: String,
}
