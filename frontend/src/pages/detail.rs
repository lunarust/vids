use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast,UnwrapThrowExt};
use crate::pages::video::Video;


#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Props {
//    pub id: usize,
    pub name: AttrValue,
//    pub path: String,
//    pub what: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct VideoDetailsProps {
    video: Video
}

#[component]
fn VideoDetails(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {

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
       let testurl = format!("http://bors.greece.local:9000/remove/{}", cloned_vid.name);
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
    }
    );

    html! {
        <div id="content">
             <iframe width="1100" height="840"
                src={ video.url.clone() }>
            </iframe>

             <span id="action">
             <button onclick={set_delete} class="button" >{ "Delete" }</button>
             <button onclick={set_archive} class="button" >{ "Archive" }</button>
             <button class="button" >{ "Turn to Gif" }</button>
             <button class="button" >{ "Mute it" }</button>
             <button class="button" >{ "Extract sound" }</button>
             </span>
        </div>
    }
}

#[function_component]
pub fn Detail(props: &Props) -> Html {
    web_sys::console::log_1(&props.name.to_string().into());

    let vid: Video = Video{id: 0, name: props.name.clone().into(),
         path: "".to_string().into(), what: "".to_string().into(), url: props.url.clone().into() };

    html!{
        <>
        <div id="content">
           <h3> {">>"} { &props.name } {"<<"}</h3>
                <VideoDetails video={vid.clone()} />
        </div>
        </>
    }

}
