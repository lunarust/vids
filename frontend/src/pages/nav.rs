use yew::prelude::*;
use crate::pages::videos::Videos;

#[function_component]
pub fn Nav() -> Html {
    //let dir = use_state(|| "archive".to_string());
    let dir = use_state(|| "backup".to_string());

    let dir = dir.clone();
    let dir_value = dir.clone();
    let toggle_dir = Callback::from(move|_| {
  //      let dir = dir.clone();
        let dir_value = dir_value.clone();

        match dir_value.as_str() {
            "backup" =>  dir_value.set("backup".to_string().into()),
             "backup" => dir_value.set("raw".to_string().into()),
             _ =>  dir_value.set("backup".to_string().into())
        }

    });
//    web_sys::console::log_1(&"Nav".into());
//    web_sys::console::log_1(&dir.to_string().into());

    if dir.as_str() == "" { dir.set("raw".to_string()); }


    html!{
        <>
            <div class="top_right">

            <button class="button" onclick={ toggle_dir } >{ dir.to_string() }</button>

            </div>

            //{ dir.to_string() }
            //{ (*dir).clone() }

            if let directory = &*dir {
                /*<span>
                {"Directory="}{directory}<br />
                {"dir="}{dir.to_string()}<br />
                {"*dir="}{(&*dir).clone().to_string()}<br />
                </span>
                */
                <Videos dir={ directory.clone() } />
            }
        </>
    }
}
