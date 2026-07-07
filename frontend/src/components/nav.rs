use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="main-nav">
            <Link<Route> to={Route::Home} classes="nav-link">{ "Accueil" }</Link<Route>>
            <Link<Route> to={Route::Explorer} classes="nav-link">{ "Explorateur" }</Link<Route>>
            <Link<Route> to={Route::NewTransaction} classes="nav-link">{ "Nouvelle transaction" }</Link<Route>>
        </nav>
    }
}
