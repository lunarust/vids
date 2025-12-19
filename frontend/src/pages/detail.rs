use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast,UnwrapThrowExt};
use crate::pages::video::Video;

use serde::Serialize;
use common::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
//    pub id: usize,
    pub name: AttrValue,
    pub path: String,
//    pub what: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct VideoDetailsProps {
    video: Video
}

#[component]
fn VideoDetails(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    let message = use_state(|| "".to_string());
    web_sys::console::log_1(&"INIT :: ".into());
    web_sys::console::log_1(&video.name.to_string().into());

   let cloned_vid = video.clone();
   //let newvideo = video.clone();
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
       let testurl = format!("/api/v1/remove/{}", cloned_vid.name);
        web_sys::console::log_1(&"Hello World I want to remove :: ".into());
        web_sys::console::log_1(&cloned_vid.name.to_string().into());
        spawn_local(async move {
            let _ = Request::post(testurl.as_str())
            .header("Content-Type", "application/json")
            .send()
            .await;
        });
        //newvideo = videos[video.id+1];
    });



   let vidname = arch_vid.name.clone();
   let vidpath = arch_vid.path.clone();
   let value = vidname.clone();
   let set_archive = Callback::from(move |_| {
       let vidname = value.clone();
       let testurl = format!("/api/v1/archive/{}", vidname);
        web_sys::console::log_1(&"Hello World I want to archive :: ".into());
        web_sys::console::log_1(&arch_vid.name.to_string().into());
        spawn_local(async move {
            let _ = Request::post(testurl.as_str())
            .header("Content-Type", "application/json")
            .send()
            .await;
        });
    }
    );

   let valuename = vidname.clone();
   let valuepath = vidpath.clone();
   let extract_sound = Callback::from(move |_| {
       let testurl = format!("/api/v1/extractsound");
       //let message = message.clone();

        let vidname = valuename.clone();
        let vidpath = valuepath.clone();
        spawn_local(async move {
            let vidreq: VideoRequest = common::VideoRequest{name: vidname.to_string(),path: vidpath.to_string()};

            let jsonbody = serde_json::to_string(&vidreq).expect("Failed");

            web_sys::console::log_1(&jsonbody.to_string().into());
            let _ = Request::post(testurl.as_str())
            .header("Content-Type", "application/json")
            .body(jsonbody.to_string()).expect("DRAMA")
            .send()
            .await;
            /*
            match response {
                Ok(resp) if resp.ok() => message.set(format!("OK").into()),
                _ => message.set(format!("Failed {:?}", response).into()),
            }*/

        });
    });

   let valuename = vidname.clone();
   let valuepath = vidpath.clone();

   let remove_sound = Callback::from(move |_| {
       let testurl = format!("/api/v1/soundout");
       //let message = message.clone();

        let vidname = valuename.clone();
        let vidpath = valuepath.clone();
        spawn_local(async move {
            let vidreq: VideoRequest = common::VideoRequest{name: vidname.to_string(),path: vidpath.to_string()};

            let jsonbody = serde_json::to_string(&vidreq).expect("Failed");

            web_sys::console::log_1(&jsonbody.to_string().into());
            let _ = Request::post(testurl.as_str())
            .header("Content-Type", "application/json")
            .body(jsonbody.to_string()).expect("DRAMA")
            .send()
            .await;
            /*
            match response {
                Ok(resp) if resp.ok() => message.set(format!("OK").into()),
                _ => message.set(format!("Failed {:?}", response).into()),
            }*/

        });
    });

   let to_gif = Callback::from(move |_| {
       let testurl = format!("/api/v1/togif");
       let message = message.clone();

        let vidname = vidname.clone();
        let vidpath = vidpath.clone();
        spawn_local(async move {
            let vidreq: VideoRequest = common::VideoRequest{name: vidname.to_string(),path: vidpath.to_string()};

            let jsonbody = serde_json::to_string(&vidreq).expect("Failed");

            web_sys::console::log_1(&jsonbody.to_string().into());
            let _ = Request::post(testurl.as_str())
            .header("Content-Type", "application/json")
            .body(jsonbody.to_string()).expect("DRAMA")
            .send()
            .await;
            /*
            match response {
                Ok(resp) if resp.ok() => message.set(format!("OK").into()),
                _ => message.set(format!("Failed {:?}", response).into()),
            }*/

        });
    });

    html! {
        <div id="content">
             <iframe width="1100" height="840"
                src={ video.url.clone() }>
            </iframe>

             <span id="action">
             <button onclick={ set_delete } class="button" >{ "Delete" }</button>
             <button onclick={ set_archive } class="button" >{ "Archive" }</button>
             <button class="button" onclick={ to_gif } >{ "Turn to Gif" }</button>
             <button class="button" onclick={ remove_sound } >{ "Mute it" }</button>
             <button onclick={ extract_sound } class="button" >{ "Extract sound" }</button>
             </span>
        </div>
    }
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    web_sys::console::log_1(&props.name.to_string().into());

    let vid: Video = Video{id: 0, name: props.name.clone().into(),
         path: props.path.clone().into(), what: "".to_string().into(), url: props.url.clone().into() };

    html!{
        <>
        <div id="content">
           <h3> { &props.name } </h3>
                <VideoDetails video={vid.clone()} />
        </div>
        </>
    }

}
