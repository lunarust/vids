use yew::prelude::*;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
//use crate::pages::video::Video;
use crate::pages::detail::Detail;

use common::*;

#[derive(Clone, Copy, Debug, EnumIter, Display, PartialEq, Serialize, Deserialize, Eq)]
pub enum Filter {
    All,
    Backup,
    Raw,
}
impl Filter {
/*    pub fn fits(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
*/
    pub fn as_href(&self) -> &'static str {
        match self {
            Filter::All => "#/",
            Filter::Raw => "#/raw",
            Filter::Backup => "#/backup",
        }
    }
}
#[derive(Properties, PartialEq)]
struct ShowListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[component]
fn ShowList(ShowListProps { videos, on_click }: &ShowListProps) -> Html {
    let on_select = |video: &Video| {
        let on_click = on_click.clone();
        let video = video.clone();
        Callback::from(move |_| {
            on_click.emit(video.clone())
        })
    };

    html! {
        <div class="navigation">
        <span class="title">{ "Videos" }</span><br /><br />
        for video in videos {
            <span key={video.id} onclick={on_select(video)} class={format!("menu_item_{}", video.archived.to_string())}
             >{format!("{}", video.name)}</span><br />
        }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub dir: String,
}

#[function_component]
pub fn Videos(props: &Props) -> Html {
    let message = use_state(|| "".to_string());
    let tot_files = use_state(|| "".to_string());
    //web_sys::console::log_1(&props.dir.to_string().into());
    let myvideo_name = use_state(|| "".to_string());
    let videos = use_state(|| vec![]);
    let dir_prop = use_state(|| props.dir.clone().to_string());

    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };


    //web_sys::console::log_1(&"Loading videos".into());

    {
        let videos = videos.clone();
        let message = message.clone();
        let tot_files = tot_files.clone();
        let myvideo_val = selected_video.clone();

        let value = message.clone();
        let videosval = videos.clone();
        let tot_files_val = tot_files.clone();
        let _mydir = dir_prop.clone();
        use_effect_with((), move |_| {
            let mut fetched_videos: Vec<Video> = vec![];
            let myvideo_val = myvideo_val.clone();
            spawn_local(async move {
                let response = Request::get(format!("/api/v1/list/").as_str())
                    .header("Content-Type", "application/json")
                    .send()
                    .await;
                    match response {
                        Ok(resp) if resp.ok() => {
                            fetched_videos = resp.json().await.unwrap();
                            fetched_videos.sort_by_key(|d| d.name.clone());
                            tot_files_val.set(fetched_videos.len().to_string());
                            if fetched_videos.len() > 0 {
                                myvideo_val.set(fetched_videos[0].clone().into());
                                web_sys::console::log_1(&fetched_videos[0].name.to_string().into());
                            }
                            value.set(format!("OK > {:?} {:?} [{:?} videos available]", dir_prop.to_string(), resp, fetched_videos.len()));

                        }
                        _ => value.set(format!("Failed {:?} {:?}", response, dir_prop.to_string()).into()),
                    }
                    videosval.set(fetched_videos);
            });

            || {}
        });

        let next_video_name = myvideo_name.clone();
        let videos_list = videos.clone();
        let selected_video_next = selected_video.clone();

        let go_next = Callback::from(move |direction: &str| {
            let videos_list_val = videos_list.clone();
            let mut index = 0;
            if let Some(video) = &*selected_video_next {
                let n = &*video.name;
                match direction {
                    "next" => index = videos_list.iter().position(|r| r.name == n).unwrap()+1,
                    "prev" => index = videos_list.iter().position(|r| r.name == n).unwrap()-1,
                    _ => index = videos_list.iter().position(|r| r.name == n).unwrap()+1,
                }
            }
            else {
                index = videos_list.iter().position(|r| r.name == next_video_name.to_string()).unwrap()+1;
           }
           selected_video_next.set(Some(videos_list_val[index].clone()));
           //web_sys::console::log_1(&index.to_string().into());
        });

        let value_gonext = go_next.clone();

        html!{
            <>
            <ShowList videos={(*videos).clone()} on_click={on_video_select} />

            if let Some(video) = &*selected_video {

                <div class="top">
                    <button class="button" onclick={ { move |_|{ go_next.emit("prev");}} } >{" << "}</button>
                    <span class="title">{ video.name.to_string() }</span>
                    <button class="button" onclick={ { move |_|{ value_gonext.emit("next");}} } >{" >> "}</button>
                </div>

                <Detail name={ video.name.to_string() } url={ video.url.to_string() } path={ video.path.to_string() } />
            }

            if !message.is_empty() {
                <p class="footer">{ &*message }</p>
            }

            if !tot_files.is_empty() {
                <p class="watermark">{ &*tot_files }</p>
            }
            else {
                <p class="watermark">{ "Nothing" }</p>
            }
            </>
        }
    }
}
