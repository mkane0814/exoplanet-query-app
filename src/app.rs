use crate::{
    api::QueryDb,
    model::{data::Data, input::Input},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

type InputHolder = Vec<(usize, (ReadSignal<Input>, WriteSignal<Input>))>;

#[derive(Clone, Copy)]
struct InputUpdater {
    set_input_objects: WriteSignal<InputHolder>,
}

#[derive(Clone, Copy)]
struct Fields {
    fields: ReadSignal<Vec<Item>>,
}

#[derive(Clone, Copy)]
struct QueryOutput {
    value: ReadSignal<Option<Result<Vec<Data>, ServerFnError>>>,
}

#[derive(Clone, Copy)]
pub struct Item {
    pub id: &'static str,
    pub value: &'static str,
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Link rel="icon" type_="image/x-icon" href="/assets/favicon.ico"/>
        <div class="bg-neutral"></div>
        // sets the document title
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let initial_fields = vec![
        Item {
            id: "pl_name",
            value: "Planet Name",
        },
        Item {
            id: "hostname",
            value: "Host Name",
        },
        Item {
            id: "sy_snum",
            value: "Number of Stars",
        },
        Item {
            id: "sy_pnum",
            value: "Number of Planets",
        },
        Item {
            id: "sy_mnum",
            value: "Number of Moons",
        },
        Item {
            id: "cb_flag",
            value: "Circumbinary Flag",
        },
        Item {
            id: "discovery_method",
            value: "Discovery Method",
        },
        Item {
            id: "disc_year",
            value: "Discovery Year",
        },
        Item {
            id: "disc_refname",
            value: "Discovery Reference",
        },
        Item {
            id: "disc_pubdate",
            value: "Discovery Publication Date",
        },
    ];

    let (fields, _) = create_signal(initial_fields);
    let query_action = create_server_action::<QueryDb>();
    provide_context(Fields { fields });
    provide_context(QueryOutput {
        value: query_action.value().read_only(),
    });
    view! {
        <Title text="Exoplanet Query App"/>
        <InputArea query_action/>
        <OutputArea/>
    }
}

#[component]
pub fn InputArea(query_action: Action<QueryDb, Result<Vec<Data>, ServerFnError>>) -> impl IntoView {
    let initial_size = 1;
    let mut next_counter_id = initial_size;

    let initial_inputs = (0..initial_size)
        .map(|id| (id, create_signal(Input::new())))
        .collect::<Vec<_>>();

    let (input_objects, set_input_objects) = create_signal(initial_inputs);

    provide_context(InputUpdater { set_input_objects });

    let add_input = move |_| {
        let sig = create_signal(Input::new());

        set_input_objects.update(move |inputs| inputs.push((next_counter_id, sig)));

        next_counter_id += 1;
    };

    let clear_input = move |_| {
        set_input_objects.update(|inputs| inputs.clear());
    };

    let submit_handler = move |_| {
        let mut inputs = Vec::<Input>::new();
        for (_id, (rs, _ws)) in input_objects.get() {
            inputs.push(rs.get());
        }
        query_action.dispatch(QueryDb { query: inputs });
    };

    view! {
        <div class="input-area">
            <div class="flex justify-center input-controls">
                <button class="btn btn-outline btn-secondary" on:click=add_input>
                    "Add Input"
                </button>
            </div>
            <For
                each=input_objects
                key=|input_objects| input_objects.0
                view=move |(id, (_, ws))| {
                    view! { <InputRow id=id writer=ws/> }
                }
            />

            <div class="flex submit-area justify-center gap-4">
                <button class="btn btn-outline btn-success" on:click=submit_handler>
                    "Submit"
                </button>
                <button class="btn btn-outline btn-error" on:click=clear_input>
                    "Clear Input"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn InputRow(id: usize, writer: WriteSignal<Input>) -> impl IntoView {
    let initial_comp_ops = vec![
        Item {
            id: "$eq",
            value: "=",
        },
        Item {
            id: "$ne",
            value: "!=",
        },
        Item {
            id: "$lt",
            value: "<",
        },
        Item {
            id: "$gt",
            value: ">",
        },
        Item {
            id: "$gte",
            value: ">=",
        },
        Item {
            id: "$lte",
            value: "<=",
        },
    ];

    let Fields { fields } = use_context().unwrap();

    let (comp_ops, _set_comp_ops) = create_signal(initial_comp_ops);

    let (selected_comp_op, set_selected_comp_op) = create_signal(Item {
        id: "default",
        value: "Select an Operator",
    });
    let (selected_field, set_selected_field) = create_signal(Item {
        id: "default",
        value: "Select a Field",
    });
    let InputUpdater { set_input_objects } = use_context().unwrap();

    create_effect(move |_| {
        writer.update(move |input| input.comparison_op = selected_comp_op.get().id.to_string());
        writer.update(move |input| input.field = selected_field.get().id.to_string());
    });

    view! {
        <div class="input-row" id=id>
            <Dropdown items=fields selected=selected_field set_selected=set_selected_field/>
            <Dropdown items=comp_ops selected=selected_comp_op set_selected=set_selected_comp_op/>
            <input
                class="input input-bordered input-info w-full"
                type="text"
                on:input=move |ev| {
                    writer.update(move |input| input.value = event_target_value(&ev))
                }
            />

            <button
                class="btn btn-error"
                on:click=move |_| {
                    set_input_objects
                        .update(move |inputs| inputs.retain(|(input_id, _)| input_id != &id))
                }
            >

                "x"
            </button>
        </div>
    }
}

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
    let unwrap_data = move || match value.get() {
        Some(wrapped_data) => match wrapped_data {
            Ok(data) => data,
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
                        key=|field| field.value
                        view=move |field| {
                            view! { <th>{move || { field.value }}</th> }
                        }
                    />

                </tr>
            </thead>
            <tbody>
                <For
                    each=unwrap_data
                    key=|result| result.to_owned()
                    view=move |result| {
                        view! { <SummaryRow data=result/> }
                    }
                />

            </tbody>
        </table>
    }
}

#[component]
pub fn SummaryRow(data: Data) -> impl IntoView {
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
            <td>{data.sy_mnum}</td>
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
                    <div class="grid grid-cols-10 auto-cols-max gap-4 grid-flow-col">
                        <div class="grid grid-cols-1">
                            <div>"Planet Letter"</div>
                            <div>{data.pl_letter.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1">
                            <div>"Orbital Period [days]"</div>
                            <SupSub text={data.pl_orbper.to_owned()} sup={data.pl_orbpererr1.to_owned()} sub={data.pl_orbpererr2.to_owned()} />
                        </div>
                        <div class="grid grid-cols-1">
                            <div>"Planet Radius [Earth Radius]"</div>
                            <SupSub text={data.pl_rade.to_owned()} sup={data.pl_radeerr1.to_owned()} sub={data.pl_radeerr2.to_owned()} />
                        </div>
                        <div class="grid grid-cols-1">
                            <div>"Planet Mass [Earth Mass]"</div>
                            <SupSub text={data.pl_bmasse.to_owned()} sup={data.pl_bmasseerr1.to_owned()} sub={data.pl_bmasseerr2.to_owned()} />
                        </div>
                        <div class="grid grid-cols-1">
                            <div>"Planet Mass Estimation Formula"</div>
                            <div>{data.pl_bmassprov.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1">
                            <div>"Discovery Facility"</div>
                            <div>{data.disc_facility.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1">
                            <div>"Spectral Type"</div>
                            <div>{data.st_spectype.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1">
                            <div>"Discovery Telescope"</div>
                            <div>{data.disc_telescope.to_owned()}</div>
                        </div>
                        <div class="grid grid-cols-1">
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

#[component]
pub fn Dropdown(
    items: ReadSignal<Vec<Item>>,
    selected: ReadSignal<Item>,
    set_selected: WriteSignal<Item>,
) -> impl IntoView {
    let (open, set_open) = create_signal(false);
    view! {
        <details class="dropdown" value=move || selected().id.to_string() prop:open=open>
            <summary class="btn btn-outline btn-info m-1">
                {move || selected().value.to_string()}
            </summary>
            <ul class="dropdown-content z-[1] menu shadow bg-base-200 rounded-box w-52">
                <For
                    each=items
                    key=|item| item.id
                    view=move |item| {
                        view! {
                            <li
                                value=item.id
                                on:click=move |_| {
                                    set_open(false);
                                    set_selected(item);
                                }
                            >

                                {move || item.value}
                            </li>
                        }
                    }
                />

            </ul>
        </details>
    }
}
