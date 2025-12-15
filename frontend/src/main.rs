#[warn(unused_variables)]

use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast,UnwrapThrowExt};

#[derive(Clone, PartialEq, Deserialize, Debug)]
struct Video {
    id: usize,
    name: AttrValue,
    path: AttrValue,
    what: AttrValue,
    url: AttrValue,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[component]
fn VideosList(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_select = |video: &Video| {
        let on_click = on_click.clone();
        let video = video.clone();
        Callback::from(move |_| {
            on_click.emit(video.clone())
        })
    };
    html! {
        for video in videos {
            <span key={video.id} onclick={on_select(video)} class="menu_item">{format!("{}", video.name)}</span><br />
        }
    }
}
#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
//    work_state:  Callback<String>,
}

#[component]
fn VideoDetails(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
   let cloned_vid = video.clone();
 let arch_vid = video.clone();
   let _oninput = Callback::from({
       move |input_event: InputEvent| {
           let target: web_sys::HtmlInputElement = input_event
           .target()
           .unwrap_throw()
           .dyn_into()
           .unwrap_throw();
           web_sys::console::log_1(&target.value().into());
       }
   });

   let set_delete = Callback::from(move |_| {
       let testurl = format!("http://bors.greece.local:9000/remove/{}", cloned_vid.name);
        web_sys::console::log_1(&"Hello World I want to remove :: ".into());
        web_sys::console::log_1(&cloned_vid.name.to_string().into());
        spawn_local(async move {
            let _ = Request::post(testurl.as_str())
            .header("Content-Type", "application/json")
            .send()
            .await;
        });
    });


   let set_archive = Callback::from(move |_| {
       let testurl = format!("http://bors.greece.local:9000/archive/{}", arch_vid.name);
        web_sys::console::log_1(&"Hello World I want to archive :: ".into());
        web_sys::console::log_1(&arch_vid.name.to_string().into());
        spawn_local(async move {
            let _ = Request::post(testurl.as_str())
            .header("Content-Type", "application/json")
            .send()
            .await;
        });
    });

    html! {
        <div id="content">
            <h3>{ &*video.name } {">>"}
            </h3>

             <iframe width="800" height="600"
                src={ video.url.clone() }>
            </iframe>

             <span id="action">
             <button onclick={set_delete} class="button" >{ "Delete" }</button>
             <button onclick={set_archive} class="button" >{ "Archive" }</button>
             <button class="button" >{ "Turn to Gif" }</button>
             </span>
        </div>
    }
}


#[function_component(App)]
fn app() -> Html {
   let message = use_state(|| "".to_string());

   let videos = use_state(|| vec![]);
   {
       let videos = videos.clone();
       let message = message.clone();
       use_effect_with((), move |_| {
           let mut fetched_videos: Vec<Video> = vec![];
           wasm_bindgen_futures::spawn_local(async move {
               let response =
               Request::get("http://bors.greece.local:9000/list")
                   .header("Content-Type", "application/json")
                   .send()
                   .await;
               match response {
                   Ok(resp) if resp.ok() => {
                       fetched_videos = resp.json().await.unwrap();
                       message.set(format!("Success {:?} [{:?} videos available]", resp, fetched_videos.len()).into());
                   }
                   _ => message.set(format!("Failed {:?}", response).into()),
               }
               videos.set(fetched_videos);

           });
           || ()
       });

   }

    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    html! {
        <>
            <div class="navigation">
            <h3>{ "Videos list" }</h3>
            <VideosList videos={(*videos).clone()} on_click={on_video_select} />
            </div>

            if let Some(video) = &*selected_video {
                <VideoDetails video={video.clone()}  />
            }


            if !message.is_empty() {
            <p class="footer">{ &*message }</p>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
