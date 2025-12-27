use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

//use crate::pages::home::Home;
//use crate::pages::videos::Videos;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::loading::Loading;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}
#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
           // <Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>
            /*
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
            */
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}


#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            //<Nav />

            <main>
                <Switch<Route> render={switch} />
            </main>
            /*
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
            */
        </Router>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Loading /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}
