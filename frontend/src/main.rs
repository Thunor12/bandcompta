mod components;
mod pages;
mod ui;

use components::Nav;
use pages::{ContactsPage, Explorer, Home, NewTransactionPage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/explorer")]
    Explorer,
    #[at("/new")]
    NewTransaction,
    #[at("/contacts")]
    Contacts,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Explorer => html! { <Explorer /> },
        Route::NewTransaction => html! { <NewTransactionPage /> },
        Route::Contacts => html! { <ContactsPage /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <HashRouter>
            <Nav />
            <main>
                <Switch<Route> render={switch} />
            </main>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
