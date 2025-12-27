use yew::prelude::*;
use yew::suspense::{use_future};
use gloo_net::http::Request;

use common::*; // Video is defined here
use crate::pages::phone::Phone;
use crate::pages::watch::Watch;

#[derive(Properties, PartialEq)]
struct ShowListProps {
    on_click: Callback<Video>,
}
#[function_component(VideosDisplay)]
fn video_display( ShowListProps { on_click }: &ShowListProps ) -> HtmlResult {

    // use_future automatically handles the async request and suspends
    let videos_handle = use_future(|| async {
        Request::get("/api/v1/list/")
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<Vec<Video>>()
            .await
    })?; // The '?' operator suspends the component if the future isn't ready

    // Initialize local state with the fetched data to allow filtering
    let videos_all = use_state(|| match &*videos_handle {
        Ok(v) => v.clone(),
        Err(_) => vec![],
    });

    // Handle the initial fetch error
    if let Err(e) = &*videos_handle {
        return Ok(html! { <p class="footer">{"Error loading videos: "}{e}</p> });
    }
    let show_archived = use_state(|| false);


    let displayed_videos = videos_all.iter().filter(|v| {
       if *show_archived {
           v.archived // If checkbox checked, only show archived
       } else {
           !v.archived // If unchecked, only show active
       }
    });


    let on_toggle = {
        let show_archived = show_archived.clone();
        Callback::from(move |_| show_archived.set(!*show_archived))
    };


    // Preparing onclick logic to emit on each video
    let on_select = |video: &Video| {
        let on_click = on_click.clone();
        let video = video.clone();

        Callback::from(move |_| {
            on_click.emit(video.clone())

        })
    };


    Ok(
        html!
        {
        <div>
            <h3>{ format!("Total {} ", videos_all.len()) }</h3>
        <ul>
            { for displayed_videos.map(|video| {

                html! {
                    <li key={video.id}
                        onclick={on_select(video)}
                        class={format!("menu_item_{}", video.archived)}>
                        { &video.name }
                    </li>
                }
            })}
        </ul>
            <input
                type="checkbox"
                id="toggle-all"
                checked={*show_archived}
                onclick={on_toggle}
            />
            <label for="toggle-all">{ " Show Archived " }</label>
        </div>
    })


}

#[function_component(Loading)]
pub fn loading() -> Html {

    //Callback when a video was selected after onclick on the list
    let selected_video = use_state(|| None);
    let on_video_select = {

       let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let fallback = html! { <div>{"Loading video data..."}</div> };

    html! {
        <>
        //Phone component to allow import of new surveillance videos onto the backend
        <Phone />
        <div>

            //The list of videos must be loaded from the Backend using api call
            <Suspense {fallback}>
                <VideosDisplay on_click={on_video_select} />
            </Suspense>

            //If a video has been selected, loading it to be watched
            if let Some(video) = &*selected_video {
                <div class="top">

                    <span class="title">{ video.name.to_string() } </span>
                    <Watch vid={ video.clone() } />

                </div>
            }
        </div>
        </>
    }
}
