use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use crate::pages::detail::Detail;

#[derive(Clone, PartialEq, Deserialize, Debug, Eq, Ord, PartialOrd)]
struct Video {
    id: usize,
    name: AttrValue,
    path: AttrValue,
    what: AttrValue,
    url: AttrValue,
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
        for video in videos {
            <span key={video.id} onclick={on_select(video)} class="menu_item">{format!("{}", video.name)}</span><br />
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
    web_sys::console::log_1(&props.dir.to_string().into());
    let myvideo_name = use_state(|| "".to_string());
    let videos = use_state(|| vec![]);
    let dir = use_state(|| props.dir.clone().to_string());

    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };



    {
        let videos = videos.clone();
        let message = message.clone();
        let tot_files = tot_files.clone();
        let myvideo_val = selected_video.clone();

        let value = message.clone();
        let videosval = videos.clone();
        let tot_files_val = tot_files.clone();

        use_effect_with((), move |_| {
            let mut fetched_videos: Vec<Video> = vec![];
            let myvideo_val = myvideo_val.clone();
            spawn_local(async move {
                let response = Request::get("http://bors.greece.local:9000/list")
                    .header("Content-Type", "application/json")
                    .send()
                    .await;
                    match response {
                        Ok(resp) if resp.ok() => {
                            fetched_videos = resp.json().await.unwrap();
                            fetched_videos.sort_by_key(|d| d.name.clone());
                            tot_files_val.set(fetched_videos.len().to_string());
                            myvideo_val.set(fetched_videos[0].clone().into());
                            web_sys::console::log_1(&fetched_videos[0].name.to_string().into());
                            value.set(format!("Success {:?} [{:?} videos available]", resp, fetched_videos.len()));

                        }
                        _ => value.set(format!("Failed {:?}", response).into()),
                    }
                    videosval.set(fetched_videos);


            });

            || {}
        });


        let next_video_name = myvideo_name.clone();
        let videos_list = videos.clone();
        let selected_video_next = selected_video.clone();

        let go_next = Callback::from(move |_| {
            let videos_list_val = videos_list.clone();
            let mut index = 0;
            if let Some(video) = &*selected_video_next {
                let n = &*video.name;
                index = videos_list.iter().position(|r| r.name == n).unwrap()+1;
                //web_sys::console::log_1(&"I use selected video".into());
            }
            else {
                index = videos_list.iter().position(|r| r.name == &next_video_name.to_string()).unwrap()+1;
                //web_sys::console::log_1(&"I use default video".into());
           }
           selected_video_next.set(Some(videos_list_val[index].clone()));
           //web_sys::console::log_1(&index.to_string().into());
        });


        let dir = dir.clone();
        let dir_value = dir.clone();
        let toggle_dir = Callback::from(move|_| {
            let mut dir_value = dir_value.clone();

            if dir_value.to_string() == "raw".to_string() {
                dir_value.set("archive".to_string().into());
            }
            else { dir_value.set("raw".to_string().into()); }


        });

        html!{
            <>
            <div class="top">
            <button class="button" onclick={ toggle_dir } >{ dir.to_string() }</button>
            <button class="button" onclick={ go_next } >{" >> "}</button>
            </div>

            <ShowList videos={(*videos).clone()} on_click={on_video_select} />


            if let Some(video) = &*selected_video {
                <Detail name={ video.name.to_string() } url={ video.url.to_string() } />
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
