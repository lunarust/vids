use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, PartialEq)]
pub struct Mobile {
    name: String,
    ip: String,
}
#[derive(Properties, PartialEq)]
struct MobilesListProps {
    mobiles: Vec<Mobile>,
    on_click: Callback<Mobile>,
}

#[component]
fn MobilesList(MobilesListProps { mobiles, on_click }: &MobilesListProps) -> Html {
    let fetch_phone_click = |mobile: &Mobile| {
        let on_click = on_click.clone();
        let mobile = mobile.clone();
        Callback::from(move |_| {
            on_click.emit(mobile.clone())
        })
    };

    html!{
        for mobile in mobiles {
            <span class="title">{ mobile.name.clone() }</span><br />
                <button class="button" onclick={fetch_phone_click(mobile)}>{ " Fetch " }</button><br />
            <br /><br />
        }
    }
}


#[function_component]
pub fn Phone() -> Html {
    let mobiles = vec![
        Mobile {name: "K61".to_string(), ip: "192.168.1.230".to_string()},
        Mobile {name: "Redmi".to_string(), ip: "192.168.1.208".to_string()},
    ];
    //let args = ["".to_string(), "".to_string()];
    let selected_mobile = use_state(|| None);
    let on_phone_click = {
        let selected_mobile = selected_mobile.clone();
        Callback::from(move |mobile: Mobile| {
            selected_mobile.set(Some(mobile))

        })
    };

    if let Some(mobile) = &*selected_mobile {
        web_sys::console::log_1(&mobile.clone().name.into());
        let mobile_value = mobile.clone();
        let testurl = format!("/api/v1/fetch/{}", mobile_value.ip);
        spawn_local(async move{
            let _ = Request::get(testurl.clone().as_str())
                .header("Content-Type", "application/json")
                .send()
                .await;
        web_sys::console::log_1(&testurl.into());

        });

    }
    html!{
        <div id="mobile">
        <hr /><br />
            <span class="title">{ "Phones" }</span>
            <br />{"--"}<br />
            <MobilesList {mobiles} on_click={on_phone_click} />
        <hr />
        </div>
    }
}
