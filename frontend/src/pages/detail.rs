use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast,UnwrapThrowExt};
use crate::pages::video::Video;

//use serde::Serialize;
use common::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub path: String,
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

   //let cloned_vid = video.clone();
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


   let vidname = arch_vid.name.clone();
   let vidpath = arch_vid.path.clone();
   //let value = vidname.clone();

   let handle_file_click = Callback::from(move |action: &str| {
       let mut testurl = format!("/api/v1/");
       match action {
           "gif" => testurl = format!("{}/togif", testurl),
           "soundout" => testurl = format!("{}/soundout", testurl),
           "extractsound" => testurl = format!("{}/extractsound", testurl),
           "delete" => testurl = format!("{}/remove", testurl),
           "archive"  => testurl = format!("{}/archive", testurl),
           _ => testurl = format!("{}/donothing", testurl),
       }

        let _message = message.clone();
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

    let value_soundout = handle_file_click.clone();
    let value_extractsound = handle_file_click.clone();
    let value_delete = handle_file_click.clone();
    let value_archive = handle_file_click.clone();

    html! {
        <div id="content">
             <iframe width="1100" height="840"
                src={ video.url.clone() }>
            </iframe>

             <span id="action">
                <span class="title">{ "Actions: " }</span>
                 <button onclick={ move |_|{ handle_file_click.emit("gif");}} class="button">{ "Gif it" }</button>
                 <button onclick={ move |_|{ value_soundout.emit("soundout");}} class="button">{ "Mute it" }</button>
                 <button onclick={ move |_|{ value_extractsound.emit("extractsound");}} class="button">{ "Extract sound" }</button>
                 <br />{ "--" }<br />
                 <button onclick={ move |_|{ value_archive.emit("archive");}} class="button">{ "Archive" }</button>
                 <button onclick={ move |_|{ value_delete.emit("delete");}} class="button">{ "Delete" }</button>
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
          // <h3> { &props.name } </h3>
                <VideoDetails video={vid.clone()} />
        </>
    }

}
