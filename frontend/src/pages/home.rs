use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <section class="home-page">
            <h1>{ "Bandcompta" }</h1>
            <p class="subtitle">{ "Comptabilité simplifiée pour le groupe." }</p>

            <div class="home-grid">
                <Link<Route> to={Route::Explorer} classes="home-card">
                    <h2>{ "Explorateur" }</h2>
                    <p>{ "Parcourir les transactions, filtrer par date et consulter la trésorerie." }</p>
                </Link<Route>>

                <Link<Route> to={Route::NewTransaction} classes="home-card">
                    <h2>{ "Nouvelle transaction" }</h2>
                    <p>{ "Saisir une recette, une dépense ou une note de frais avec son justificatif." }</p>
                </Link<Route>>
            </div>
        </section>
    }
}
