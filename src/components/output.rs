#![allow(non_snake_case)]

use crate::{
    components::input::{Fields, FirstId, LastId, QueryOutput},
    model::data::PlanetData,
};
use leptos::*;

#[component]
pub fn OutputArea() -> impl IntoView {
    view! {
        <div class="output-area overflow-x-auto overflow-y-auto">
            <OutputTable/>
        </div>
    }
}

#[component]
pub fn OutputTable() -> impl IntoView {
    let Fields { fields } = use_context().unwrap();
    let QueryOutput { value } = use_context().unwrap();

    let LastId {
        last_id: _,
        set_last_id,
    } = use_context().unwrap();

    let FirstId {
        first_id: _,
        set_first_id,
    } = use_context().unwrap();

    let unwrap_data = move || match value.get() {
        Some(wrapped_data) => match wrapped_data {
            Ok(data_opt) => {
                if let Some(data) = data_opt {
                    set_last_id.update(move |last_id| *last_id = data.last_id);
                    set_first_id.update(move |first_id| *first_id = data.first_id);
                    data.planet_data
                } else {
                    Vec::new()
                }
            }
            Err(_) => Vec::new(),
        },
        None => Vec::new(),
    };

    view! {
        <table class="output-table table">
            <thead>
                <tr>
                    <For
                        each=fields
                        key=|field| field.fe_id
                        let:field
                    >
                            <th>{move || { field.value }}</th>
                    </For>

                </tr>
            </thead>
            <tbody>
                <For
                    each=unwrap_data
                    key=|result| result.to_owned()
                    let:result
                >
                       <SummaryRow data=result/>

                </For>

            </tbody>
        </table>
    }
}

#[component]
pub fn SummaryRow(data: PlanetData) -> impl IntoView {
    let (open, set_open) = create_signal(false);
    let toggle = move |_| set_open(!open());
    view! {
        <tr class="summary-row hover" on:click=toggle>
            <td>
                <a href=data.caltech_href>{data.pl_name}</a>
            </td>
            <td>{data.hostname}</td>
            <td>{data.sy_snum}</td>
            <td>{data.sy_pnum}</td>
            <td>{data.cb_flag}</td>
            <td>{data.discovery_method}</td>
            <td>{data.disc_year}</td>
            <td>
                <a href=data.disc_refhref>{data.disc_refname}</a>
            </td>
            <td>{data.disc_pubdate}</td>
        </tr>
        <Show when=open fallback=|| ()>
            <tr class="bg-primary-focus">
                <td colspan=10>
                    <div class="grid grid-flow-col">
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Planet Letter"</div>
                            <div>{data.pl_letter.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Orbital Period [days]"</div>
                            <SupSub
                                text=data.pl_orbper.to_owned()
                                sup=data.pl_orbpererr1.to_owned()
                                sub=data.pl_orbpererr2.to_owned()
                            />
                        </div>
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Planet Radius [Earth Radius]"</div>
                            <SupSub
                                text=data.pl_rade.to_owned()
                                sup=data.pl_radeerr1.to_owned()
                                sub=data.pl_radeerr2.to_owned()
                            />
                        </div>
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Planet Mass [Earth Mass]"</div>
                            <SupSub
                                text=data.pl_bmasse.to_owned()
                                sup=data.pl_bmasseerr1.to_owned()
                                sub=data.pl_bmasseerr2.to_owned()
                            />
                        </div>
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Planet Mass Estimation Formula"</div>
                            <div>{data.pl_bmassprov.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Discovery Facility"</div>
                            <div>{data.disc_facility.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Spectral Type"</div>
                            <div>{data.st_spectype.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Discovery Telescope"</div>
                            <div>{data.disc_telescope.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1 gap-[0.5rem]">
                            <div>"Release Date"</div>
                            <div>{data.release_date.to_owned()}</div>
                        </div>
                    </div>
                </td>
            </tr>
        </Show>
    }
}

#[component]
pub fn SupSub(text: Option<String>, sup: Option<String>, sub: Option<String>) -> impl IntoView {
    let unwrapped_text = text.unwrap_or("".to_string());
    let unwrapped_sub = sub.unwrap_or("".to_string());
    let unwrapped_sup = sup.unwrap_or("".to_string());

    view! {
        <div>
            {unwrapped_text} <span class="supsub">
                <sup>{unwrapped_sup}</sup>
                <sub>{unwrapped_sub}</sub>
            </span>
        </div>
    }
}
