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

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Ciphertext {
    pub rotor: i32,
    pub plain: String,
    pub cryptic: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CiphertextRequest {
    pub rotor: i32,
    pub plain: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CiphertextResponse {
    pub rotor: i32,
    pub plain: String,
    pub cryptic: String,
}

impl CiphertextResponse {
    pub fn of(ciphertext: Ciphertext) -> CiphertextResponse {
        CiphertextResponse {
            rotor: ciphertext.rotor,
            plain: ciphertext.plain,
            cryptic: ciphertext.cryptic,
        }
    }
}
