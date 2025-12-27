use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use common::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub vid: Video,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct VideoDetailsProps {
    video: Video
}

#[component]
fn VideoDetails(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
   //let _message = use_state(|| "".to_string());

   let arch_vid = video.clone();


   let vidname = arch_vid.name.clone();
   let vidpath = arch_vid.path.clone();

   let handle_file_click = Callback::from(move |action: &str| {
       let url = format!("/api/v1/{}", action);

        //let _message = message.clone();
        let vidname = vidname.clone();
        let vidpath = vidpath.clone();
        spawn_local(async move {
            let vidreq: VideoRequest = common::VideoRequest{name: vidname.to_string(),path: vidpath.to_string()};

            let jsonbody = serde_json::to_string(&vidreq).expect("Failed");

            let _ = Request::post(url.as_str())
            .header("Content-Type", "application/json")
            .body(jsonbody.to_string()).expect("DRAMA")
            .send()
            .await;

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
             <br />
                <span class="title">{ "Actions: " }</span>
                 <button onclick={ move |_|{ handle_file_click.emit("togif");}} class="button">{ "Gif it" }</button>
                 <button onclick={ move |_|{ value_soundout.emit("soundout");}} class="button">{ "Mute it" }</button>
                 <button onclick={ move |_|{ value_extractsound.emit("extractsound");}} class="button">{ "Extract sound" }</button>
                 <br />{ "--" }<br />
                 <button onclick={ move |_|{ value_archive.emit("archive");}} disabled={arch_vid.archived} class="button">{ "Archive" }</button>
                 <button onclick={ move |_|{ value_delete.emit("remove");}} class="button">{ "Delete" }</button>
             </span>
        </div>
    }
}

#[function_component]
pub fn Watch(props: &Props) -> Html {
    //web_sys::console::log_1(&props.vid.name.to_string().into());

    let vid = &props.vid;
    html!{
        <>
            <VideoDetails video={vid.clone()} />
        </>
    }
}
