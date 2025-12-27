use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use yew::events::{FocusEvent, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Classes, Component, Context, Html, NodeRef, TargetCast};
use yew::suspense::use_future;

use strum::IntoEnumIterator;

use crate::pages::state::{Filter, State};
use common::*;
fn load_videos() -> Option<Vec<Video>> {
    let mut videos: Vec<Video> = vec![];
    let mut value = videos.clone();
    spawn_local(async move {
        value = Request::get(format!("/api/v1/list/").as_str())
        .header("Content-Type", "application/json")
        .send()
        .await.expect("REASON")
        .json().await.unwrap();
        web_sys::console::log_1(&format!("LOAD.load_videos..Spawning...Loading videos {}", &value.len()).into());
    });
    Some(videos)
}
pub enum Msg {
    Add(String),
    Remove(usize),
    SetFilter(Filter),
    Focus,
}
pub struct Playground {
    state: State,
    focus_ref: NodeRef,
}

impl Component for Playground {
    type Message = Msg;
    type Properties = ();


    fn create(_ctx: &Context<Self>) -> Self {

        let mut videos: Vec<Video> = vec![];
        /*
        let videos = use_future(|| async {
          load_videos();
        })?;
        */
        let state = State::new(videos);
        let focus_ref = NodeRef::default();
        Self { state, focus_ref }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add(path) => {
                let video = Video {
                    id: 0,
                    name: path.split("/").last().expect("REASON").to_string(),
                    path: path.clone(),
                    url: path.replace("/opt/vids/", "http://localhost/vids/"),
                    archived: false,

                };
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::Focus => todo!()
            /*Msg::Focus => {
                if let Some(video) = self.focus_ref.cast::<>
            }*/
        }
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

      let fallback = html! {<div>{"Loading..."}</div>};


       // let mut videos: Vec<Video> = vec![];

       // let videos = use_video()?;

        //Ok(html! {<div>{"Hello, "}{&user.name}</div>})
        /*
        let mut message = "".to_string();
        let mut value = videos.clone();
        //use_effect_with((), move |_| {
        spawn_local(async move {
            let mut value = value.clone();
            let response = Request::get(format!("/api/v1/list/").as_str())
                .header("Content-Type", "application/json")
                .send()
                .await;
                match response {
                    Ok(resp) if resp.ok() => {
                        value = resp.json().await.unwrap();
                        value.sort_by_key(|d| d.name.clone());
                        message = format!("OK > {:?} ]", resp).to_string();
                    }
                    _ => message = format!("Failed {:?}", response).to_string(),
                }
                web_sys::console::log_1(&format!("VIEW...Spawning...Loading videos {}", &value.len()).into());
            });
       //});
       */
        web_sys::console::log_1(&format!("VIEW...self.state.videos.len {}", &self.state.videos.len()).into());


    let hidden_class = if self.state.videos.is_empty() {
        "aa"
    } else {
        ""
    };
    html!{

        <>
        //    <Content />


        <span>{ "Playground" }</span>
        <section class={classes!("main", hidden_class)}>
        <ul class="todo-list">
            { for self
                .state
                .videos
                .iter()
                .enumerate()
                .filter(|(_, video)| self.state.filter.fits(video))
                .map(|(i, e)| self.view_video((i, e), ctx.link()))
            }
        </ul>
        </section>
        <footer class={classes!("footer", hidden_class)}>
            <span class="todo-count">
                <strong>{ self.state.total() }</strong>
                { " item(s) left" }
            </span>
            <ul class="filters">
                { for Filter::iter().map(|flt| self.view_filter(flt, ctx.link())) }
            </ul>
        </footer>

         </>

    }

    }


}

impl Playground {



    fn view_filter(&self, filter: Filter, link: &Scope<Self>) -> Html {
        let cls = if self.state.filter == filter {
            "selected"
        } else {
            "not-selected"
        };
        html!{
            <li>
                 <a class={cls}
                    href={filter.as_href()}
                    onclick={link.callback(move |_| Msg::SetFilter(filter))}
                 >
                     { filter }
                 </a>
            </li>
        }
    }

    fn view_video(&self, (idx, video): (usize, &Video), link: &Scope<Self>) -> Html {

        let mut class = Classes::from("todo");
        if video.archived {
            class.push(" completed");
        }
        html! {
            <>
            <span>{"plop"}</span>
            <li {class}>
                <div class="view">
                   // <input
                   //     type="checkbox"
                   //     class="toggle"
                   //     checked={video.archived}
                    //    onclick={link.callback(move |_| Msg::Toggle(idx))}
                   // />
                    <label >{ &video.name }</label>
                    <button class="destroy" onclick={link.callback(move |_| Msg::Remove(idx))} />
                </div>
//                { self.view_entry_edit_input((idx, entry), link) }
            </li>
            </>
        }
    }


}
