use yew::prelude::*;
use serde::Deserialize;
#[derive(Clone, PartialEq, Deserialize, Debug, Eq, Ord, PartialOrd)]
pub struct Video {
    pub id: i32,
    pub name: AttrValue,
    pub path: AttrValue,
    pub what: AttrValue,
    pub url: AttrValue,
}
