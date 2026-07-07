use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConfirmDialogProps {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub on_confirm: Callback<()>,
    pub on_cancel: Callback<()>,
}

#[function_component(ConfirmDialog)]
pub fn confirm_dialog(props: &ConfirmDialogProps) -> Html {
    let on_backdrop = props.on_cancel.clone();
    html! {
        <div class="modal-backdrop" onclick={on_backdrop.reform(|_| ())}>
            <div class="modal" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <h2>{ props.title.clone() }</h2>
                <p>{ props.message.clone() }</p>
                <div class="modal-actions">
                    <button type="button" class="button-secondary" onclick={props.on_cancel.reform(|_| ())}>
                        { "Annuler" }
                    </button>
                    <button type="button" onclick={props.on_confirm.reform(|_| ())}>
                        { props.confirm_label.clone() }
                    </button>
                </div>
            </div>
        </div>
    }
}
