use bandcompta_shared::{
    AdjustStock, ApiClient, NewProduct, NewProductVariant, ProductDetail, ProductKind,
    ProductSummary, ProductVariant, VariantAttribute, VariantOption, ALBUM_FORMATS, SHIRT_SIZES,
    variant_label,
};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::ui::{input_value, parse_product_kind, select_value, format_euro, LoadState};

#[derive(Clone, PartialEq)]
struct ProductForm {
    name: String,
    kind: ProductKind,
    description: String,
}

impl Default for ProductForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            kind: ProductKind::Tshirt,
            description: String::new(),
        }
    }
}

#[derive(Clone, PartialEq, Default)]
struct VariantForm {
    sku: String,
    stock_quantity: String,
    unit_price: String,
    color: String,
    size: String,
    format: String,
    vinyl_color: String,
}

#[derive(Clone, PartialEq, Default)]
struct StockAdjustForm {
    delta: String,
    note: String,
}

#[function_component(StockPage)]
pub fn stock_page() -> Html {
    let products = use_state(LoadState::<Vec<ProductSummary>>::default);
    let selected_id = use_state(|| None::<i64>);
    let detail = use_state(LoadState::<ProductDetail>::default);
    let product_form = use_state(ProductForm::default);
    let variant_form = use_state(VariantForm::default);
    let adjust_forms = use_state(std::collections::HashMap::<i64, StockAdjustForm>::new);
    let error = use_state(String::new);
    let success = use_state(String::new);
    let submitting = use_state(|| false);

    let reload_products = {
        let products = products.clone();
        Callback::from(move |_| {
            products.set(LoadState::Loading);
            let products = products.clone();
            spawn_local(async move {
                match ApiClient::default_client().list_products().await {
                    Ok(data) => products.set(LoadState::Ready(data)),
                    Err(err) => products.set(LoadState::Error(err)),
                }
            });
        })
    };

    {
        let reload_products = reload_products.clone();
        use_effect_with((), move |_| {
            reload_products.emit(());
            || ()
        });
    }

    let reload_detail = {
        let detail = detail.clone();
        let selected_id = selected_id.clone();
        Callback::from(move |_| {
            if let Some(id) = *selected_id {
                detail.set(LoadState::Loading);
                let detail = detail.clone();
                spawn_local(async move {
                    match ApiClient::default_client().get_product(id).await {
                        Ok(data) => detail.set(LoadState::Ready(data)),
                        Err(err) => detail.set(LoadState::Error(err)),
                    }
                });
            }
        })
    };

    {
        let selected_id = selected_id.clone();
        let reload_detail = reload_detail.clone();
        use_effect_with(*selected_id, move |id| {
            if id.is_some() {
                reload_detail.emit(());
            }
            || ()
        });
    }

    let select_product = {
        let selected_id = selected_id.clone();
        let detail = detail.clone();
        let variant_form = variant_form.clone();
        Callback::from(move |id: i64| {
            selected_id.set(Some(id));
            detail.set(LoadState::Loading);
            variant_form.set(VariantForm::default());
        })
    };

    let on_create_product = {
        let product_form = product_form.clone();
        let error = error.clone();
        let success = success.clone();
        let submitting = submitting.clone();
        let reload_products = reload_products.clone();
        let selected_id = selected_id.clone();
        let reload_detail = reload_detail.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error.set(String::new());
            success.set(String::new());
            submitting.set(true);

            let payload = NewProduct {
                name: product_form.name.clone(),
                kind: product_form.kind,
                description: product_form.description.clone(),
            };

            let error = error.clone();
            let success = success.clone();
            let submitting = submitting.clone();
            let product_form = product_form.clone();
            let reload_products = reload_products.clone();
            let selected_id = selected_id.clone();
            let reload_detail = reload_detail.clone();

            spawn_local(async move {
                match ApiClient::default_client().create_product(&payload).await {
                    Ok(created) => {
                        success.set(format!("Produit « {} » créé.", created.product.name));
                        product_form.set(ProductForm::default());
                        selected_id.set(Some(created.product.id));
                        reload_products.emit(());
                        reload_detail.emit(());
                    }
                    Err(err) => error.set(err),
                }
                submitting.set(false);
            });
        })
    };

    let on_create_variant = {
        let variant_form = variant_form.clone();
        let detail = detail.clone();
        let error = error.clone();
        let success = success.clone();
        let submitting = submitting.clone();
        let reload_products = reload_products.clone();
        let reload_detail = reload_detail.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let product_id = match &*detail {
                LoadState::Ready(d) => d.product.id,
                _ => return,
            };

            error.set(String::new());
            success.set(String::new());
            submitting.set(true);

            let kind = match &*detail {
                LoadState::Ready(d) => d.product.kind,
                _ => ProductKind::Pin,
            };

            let stock_quantity = variant_form.stock_quantity.parse().unwrap_or(0);
            let unit_price = variant_form.unit_price.parse().unwrap_or(0.0);
            let attributes = build_attributes(kind, &variant_form);

            let payload = NewProductVariant {
                sku: variant_form.sku.clone(),
                stock_quantity,
                unit_price,
                attributes,
            };

            let error = error.clone();
            let success = success.clone();
            let submitting = submitting.clone();
            let variant_form = variant_form.clone();
            let reload_products = reload_products.clone();
            let reload_detail = reload_detail.clone();

            spawn_local(async move {
                match ApiClient::default_client()
                    .create_variant(product_id, &payload)
                    .await
                {
                    Ok(_) => {
                        success.set("Variante ajoutée.".into());
                        variant_form.set(VariantForm::default());
                        reload_products.emit(());
                        reload_detail.emit(());
                    }
                    Err(err) => error.set(err),
                }
                submitting.set(false);
            });
        })
    };

    let on_adjust_stock = {
        let adjust_forms = adjust_forms.clone();
        let error = error.clone();
        let success = success.clone();
        let reload_products = reload_products.clone();
        let reload_detail = reload_detail.clone();
        Callback::from(move |variant_id: i64| {
            let form = adjust_forms.get(&variant_id).cloned().unwrap_or_default();
            let delta: i32 = match form.delta.parse() {
                Ok(value) if value != 0 => value,
                _ => {
                    error.set("Indiquez une variation non nulle.".into());
                    return;
                }
            };

            error.set(String::new());
            success.set(String::new());

            let payload = AdjustStock {
                quantity_delta: delta,
                note: form.note.clone(),
            };

            let error = error.clone();
            let success = success.clone();
            let adjust_forms = adjust_forms.clone();
            let reload_products = reload_products.clone();
            let reload_detail = reload_detail.clone();

            spawn_local(async move {
                match ApiClient::default_client()
                    .adjust_stock(variant_id, &payload)
                    .await
                {
                    Ok(variant) => {
                        success.set(format!(
                            "Stock mis à jour : {} → {} unités.",
                            variant_label(&variant.attributes),
                            variant.stock_quantity
                        ));
                        let mut next = (*adjust_forms).clone();
                        next.insert(variant_id, StockAdjustForm::default());
                        adjust_forms.set(next);
                        reload_products.emit(());
                        reload_detail.emit(());
                    }
                    Err(err) => error.set(err),
                }
            });
        })
    };

    html! {
        <>
            <header class="page-header">
                <div>
                    <h1>{ "Stock merch" }</h1>
                    <p class="subtitle">{ "Produits, variantes et quantités en stock" }</p>
                </div>
                <button onclick={reload_products.reform(|_| ())}>{ "Actualiser" }</button>
            </header>

            if !(*error).is_empty() {
                <p class="error">{ (*error).clone() }</p>
            }
            if !(*success).is_empty() {
                <p class="success-panel">{ (*success).clone() }</p>
            }

            <form class="transaction-form" onsubmit={on_create_product}>
                <h2 class="form-section-title">{ "Nouveau produit" }</h2>

                <label class="form-field">
                    <span>{ "Nom" }</span>
                    <input type="text" value={product_form.name.clone()} required=true oninput={{
                        let product_form = product_form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*product_form).clone();
                            next.name = input_value(&e);
                            product_form.set(next);
                        })
                    }} />
                </label>

                <label class="form-field">
                    <span>{ "Type" }</span>
                    <select
                        value={product_kind_value(product_form.kind)}
                        onchange={{
                            let product_form = product_form.clone();
                            Callback::from(move |e: Event| {
                                let mut next = (*product_form).clone();
                                next.kind = parse_product_kind(&select_value(&e));
                                product_form.set(next);
                            })
                        }}
                    >
                        { for product_kind_options() }
                    </select>
                </label>

                <label class="form-field form-field-wide">
                    <span>{ "Description" }</span>
                    <input type="text" value={product_form.description.clone()} oninput={{
                        let product_form = product_form.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*product_form).clone();
                            next.description = input_value(&e);
                            product_form.set(next);
                        })
                    }} />
                </label>

                <div class="form-actions">
                    <button type="submit" disabled={*submitting}>{ "Créer le produit" }</button>
                </div>
            </form>

            <div class="stock-layout">
                <div class="table-panel">
                    <h2>{ "Catalogue" }</h2>
                    { match &*products {
                        LoadState::Loading => html! { <p class="muted">{ "Chargement…" }</p> },
                        LoadState::Error(err) => html! { <p class="error">{ err.clone() }</p> },
                        LoadState::Ready(items) if items.is_empty() => html! {
                            <p class="muted">{ "Aucun produit en stock." }</p>
                        },
                        LoadState::Ready(items) => html! {
                            <table>
                                <thead>
                                    <tr>
                                        <th>{ "Produit" }</th>
                                        <th>{ "Type" }</th>
                                        <th>{ "Variantes" }</th>
                                        <th>{ "Stock total" }</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    { for items.iter().map(|product| {
                                        let id = product.id;
                                        let selected = *selected_id == Some(id);
                                        let select_product = select_product.clone();
                                        html! {
                                            <tr
                                                class={if selected { "row-selected" } else { "" }}
                                                onclick={Callback::from(move |_| select_product.emit(id))}
                                            >
                                                <td>{ product.name.clone() }</td>
                                                <td>{ product.kind.label() }</td>
                                                <td>{ product.variant_count.to_string() }</td>
                                                <td>{ product.total_stock.to_string() }</td>
                                            </tr>
                                        }
                                    }) }
                                </tbody>
                            </table>
                        },
                    } }
                </div>

                <div class="stock-detail">
                    { if let Some(id) = *selected_id {
                        html! { <StockDetailPanel
                            product_id={id}
                            detail={(*detail).clone()}
                            variant_form={(*variant_form).clone()}
                            variant_form_setter={variant_form.clone()}
                            adjust_forms={(*adjust_forms).clone()}
                            adjust_forms_setter={adjust_forms.clone()}
                            on_create_variant={on_create_variant}
                            on_adjust_stock={on_adjust_stock}
                            submitting={*submitting}
                        /> }
                    } else {
                        html! { <p class="muted">{ "Sélectionnez un produit pour voir ses variantes." }</p> }
                    } }
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct StockDetailPanelProps {
    #[allow(dead_code)]
    product_id: i64,
    detail: LoadState<ProductDetail>,
    variant_form: VariantForm,
    variant_form_setter: UseStateHandle<VariantForm>,
    adjust_forms: std::collections::HashMap<i64, StockAdjustForm>,
    adjust_forms_setter: UseStateHandle<std::collections::HashMap<i64, StockAdjustForm>>,
    on_create_variant: Callback<SubmitEvent>,
    on_adjust_stock: Callback<i64>,
    submitting: bool,
}

#[function_component(StockDetailPanel)]
fn stock_detail_panel(props: &StockDetailPanelProps) -> Html {
    match &props.detail {
        LoadState::Loading => html! { <p class="muted">{ "Chargement du produit…" }</p> },
        LoadState::Error(err) => html! { <p class="error">{ err.clone() }</p> },
        LoadState::Ready(detail) => {
            let kind = detail.product.kind;
            html! {
                <>
                    <h2>{ detail.product.name.clone() }</h2>
                    <p class="muted">{ detail.product.description.clone() }</p>
                    <p>{ "Stock total : " }<strong>{ detail.total_stock.to_string() }</strong></p>

                    <div class="table-panel">
                        { if detail.variants.is_empty() {
                            html! { <p class="muted">{ "Aucune variante." }</p> }
                        } else {
                            html! {
                                <table>
                                    <thead>
                                        <tr>
                                            <th>{ "Variante" }</th>
                                            <th>{ "SKU" }</th>
                                            <th>{ "Prix" }</th>
                                            <th>{ "Stock" }</th>
                                            <th>{ "Ajuster" }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        { for detail.variants.iter().map(|variant| {
                                            render_variant_row(variant, &props.adjust_forms, &props.adjust_forms_setter, &props.on_adjust_stock)
                                        }) }
                                    </tbody>
                                </table>
                            }
                        } }
                    </div>

                    <form class="transaction-form" onsubmit={props.on_create_variant.clone()}>
                        <h3 class="form-section-title">{ "Nouvelle variante" }</h3>
                        { render_variant_fields(kind, &props.variant_form, &props.variant_form_setter) }
                        <div class="form-actions">
                            <button type="submit" disabled={props.submitting}>{ "Ajouter la variante" }</button>
                        </div>
                    </form>
                </>
            }
        }
    }
}

fn render_variant_row(
    variant: &ProductVariant,
    adjust_forms: &std::collections::HashMap<i64, StockAdjustForm>,
    adjust_forms_setter: &UseStateHandle<std::collections::HashMap<i64, StockAdjustForm>>,
    on_adjust_stock: &Callback<i64>,
) -> Html {
    let form = adjust_forms.get(&variant.id).cloned().unwrap_or_default();
    let variant_id = variant.id;
    let on_adjust = on_adjust_stock.clone();
    let adjust_forms_setter = adjust_forms_setter.clone();

    html! {
        <tr>
            <td>{ variant_label(&variant.attributes) }</td>
            <td>{ if variant.sku.is_empty() { "—".into() } else { variant.sku.clone() } }</td>
            <td>{ format_euro(variant.unit_price) }</td>
            <td><strong>{ variant.stock_quantity.to_string() }</strong></td>
            <td class="stock-adjust-cell">
                <input
                    type="text"
                    placeholder="+/-"
                    value={form.delta.clone()}
                    oninput={{
                        let adjust_forms_setter = adjust_forms_setter.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*adjust_forms_setter).clone();
                            let entry = next.entry(variant_id).or_default();
                            entry.delta = input_value(&e);
                            adjust_forms_setter.set(next);
                        })
                    }}
                />
                <input
                    type="text"
                    placeholder="Note"
                    value={form.note.clone()}
                    oninput={{
                        let adjust_forms_setter = adjust_forms_setter.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*adjust_forms_setter).clone();
                            let entry = next.entry(variant_id).or_default();
                            entry.note = input_value(&e);
                            adjust_forms_setter.set(next);
                        })
                    }}
                />
                <button type="button" onclick={Callback::from(move |_| on_adjust.emit(variant_id))}>
                    { "OK" }
                </button>
            </td>
        </tr>
    }
}

fn render_variant_fields(
    kind: ProductKind,
    form: &VariantForm,
    form_setter: &UseStateHandle<VariantForm>,
) -> Html {
    let common = html! {
        <>
            <label class="form-field">
                <span>{ "SKU" }</span>
                <input type="text" value={form.sku.clone()} oninput={{
                    let form_setter = form_setter.clone();
                    Callback::from(move |e: InputEvent| {
                        let mut next = (*form_setter).clone();
                        next.sku = input_value(&e);
                        form_setter.set(next);
                    })
                }} />
            </label>
            <label class="form-field">
                <span>{ "Stock initial" }</span>
                <input type="text" value={form.stock_quantity.clone()} oninput={{
                    let form_setter = form_setter.clone();
                    Callback::from(move |e: InputEvent| {
                        let mut next = (*form_setter).clone();
                        next.stock_quantity = input_value(&e);
                        form_setter.set(next);
                    })
                }} />
            </label>
            <label class="form-field">
                <span>{ "Prix unitaire (€)" }</span>
                <input type="text" value={form.unit_price.clone()} oninput={{
                    let form_setter = form_setter.clone();
                    Callback::from(move |e: InputEvent| {
                        let mut next = (*form_setter).clone();
                        next.unit_price = input_value(&e);
                        form_setter.set(next);
                    })
                }} />
            </label>
        </>
    };

    match kind {
        ProductKind::Tshirt => html! {
            <>
                <label class="form-field">
                    <span>{ "Couleur" }</span>
                    <input type="text" value={form.color.clone()} required=true oninput={{
                        let form_setter = form_setter.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form_setter).clone();
                            next.color = input_value(&e);
                            form_setter.set(next);
                        })
                    }} />
                </label>
                <label class="form-field">
                    <span>{ "Taille" }</span>
                    <select
                        value={form.size.clone()}
                        onchange={{
                            let form_setter = form_setter.clone();
                            Callback::from(move |e: Event| {
                                let mut next = (*form_setter).clone();
                                next.size = select_value(&e);
                                form_setter.set(next);
                            })
                        }}
                    >
                        <option value="">{ "—" }</option>
                        { for SHIRT_SIZES.iter().map(|size| html! {
                            <option value={*size}>{ *size }</option>
                        }) }
                    </select>
                </label>
                { common }
            </>
        },
        ProductKind::Album => html! {
            <>
                <label class="form-field">
                    <span>{ "Format" }</span>
                    <select
                        value={form.format.clone()}
                        onchange={{
                            let form_setter = form_setter.clone();
                            Callback::from(move |e: Event| {
                                let mut next = (*form_setter).clone();
                                next.format = select_value(&e);
                                form_setter.set(next);
                            })
                        }}
                    >
                        <option value="">{ "—" }</option>
                        { for ALBUM_FORMATS.iter().map(|format| html! {
                            <option value={*format}>{ *format }</option>
                        }) }
                    </select>
                </label>
                if form.format == "Vinyle" {
                    <label class="form-field">
                        <span>{ "Couleur vinyle" }</span>
                        <input type="text" value={form.vinyl_color.clone()} required=true oninput={{
                            let form_setter = form_setter.clone();
                            Callback::from(move |e: InputEvent| {
                                let mut next = (*form_setter).clone();
                                next.vinyl_color = input_value(&e);
                                form_setter.set(next);
                            })
                        }} />
                    </label>
                }
                { common }
            </>
        },
        ProductKind::Print => html! {
            <>
                <label class="form-field">
                    <span>{ "Format" }</span>
                    <input type="text" value={form.format.clone()} required=true oninput={{
                        let form_setter = form_setter.clone();
                        Callback::from(move |e: InputEvent| {
                            let mut next = (*form_setter).clone();
                            next.format = input_value(&e);
                            form_setter.set(next);
                        })
                    }} />
                </label>
                { common }
            </>
        },
        ProductKind::Patch | ProductKind::Pin => html! { { common } },
    }
}

fn build_attributes(kind: ProductKind, form: &VariantForm) -> Vec<VariantAttribute> {
    match kind {
        ProductKind::Tshirt => vec![
            VariantAttribute {
                option: VariantOption::Color,
                value: form.color.clone(),
            },
            VariantAttribute {
                option: VariantOption::Size,
                value: form.size.clone(),
            },
        ],
        ProductKind::Album => {
            let mut attrs = vec![VariantAttribute {
                option: VariantOption::Format,
                value: form.format.clone(),
            }];
            if form.format == "Vinyle" {
                attrs.push(VariantAttribute {
                    option: VariantOption::VinylColor,
                    value: form.vinyl_color.clone(),
                });
            }
            attrs
        }
        ProductKind::Print => vec![VariantAttribute {
            option: VariantOption::Format,
            value: form.format.clone(),
        }],
        ProductKind::Patch | ProductKind::Pin => vec![],
    }
}

fn product_kind_value(kind: ProductKind) -> &'static str {
    match kind {
        ProductKind::Tshirt => "TSHIRT",
        ProductKind::Patch => "PATCH",
        ProductKind::Pin => "PIN",
        ProductKind::Print => "PRINT",
        ProductKind::Album => "ALBUM",
    }
}

fn product_kind_options() -> Vec<Html> {
    [
        ProductKind::Tshirt,
        ProductKind::Patch,
        ProductKind::Pin,
        ProductKind::Print,
        ProductKind::Album,
    ]
    .into_iter()
    .map(|kind| {
        html! { <option value={product_kind_value(kind)}>{ kind.label() }</option> }
    })
    .collect()
}
