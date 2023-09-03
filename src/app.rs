use crate::{
    api::QueryDb,
    model::{data::Data, input::Input},
};
use leptos::*;
use leptos_meta::*;
use std::collections::HashMap;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

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
            id: "pl_letter",
            value: "Planet Letter",
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

    let (fields, _) = create_signal(cx, initial_fields);
    let query_action = create_server_action::<QueryDb>(cx);
    provide_context(cx, Fields { fields });
    provide_context(
        cx,
        QueryOutput {
            value: query_action.value().read_only(),
        },
    );

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <div class="bg-neutral"></div>
        // sets the document title
        <Title text="Welcome to stuff!"/>
        <InputArea query_action/>
        <OutputArea/>
    }
}

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
pub fn InputArea(
    cx: Scope,
    query_action: Action<QueryDb, Result<Vec<Data>, ServerFnError>>,
) -> impl IntoView {
    let initial_size = 1;
    let mut next_counter_id = initial_size;

    let initial_inputs = (0..initial_size)
        .map(|id| (id, create_signal(cx, Input::new())))
        .collect::<Vec<_>>();

    let (input_objects, set_input_objects) = create_signal(cx, initial_inputs);

    provide_context(cx, InputUpdater { set_input_objects });

    let add_input = move |_| {
        let sig = create_signal(cx, Input::new());

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

    view! { cx,
        <div class="input-area">
            <div class="input-controls">
                <button class="btn btn-outline btn-accent" on:click=add_input>"Add Input"</button>
            </div>
            <For
                each=input_objects
                key=|input_objects| input_objects.0
                view=move |cx, (id, (_, ws))| {
                    view! { cx, <InputRow id=id writer=ws/> }
                }
            />

            <div class="submit-area">
                <button class="btn btn-outline btn-primary" on:click=submit_handler>"Submit"</button>
                <button class="btn btn-outline btn-accent" on:click=clear_input>"Clear Input"</button>
            </div>
        </div>
    }
}

#[component]
pub fn InputRow(cx: Scope, id: usize, writer: WriteSignal<Input>) -> impl IntoView {
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

    let Fields { fields } = use_context(cx).unwrap();

    let (comp_ops, _set_comp_ops) = create_signal(cx, initial_comp_ops);

    let (selected_comp_op, set_selected_comp_op) = create_signal(
        cx,
        Item {
            id: "default",
            value: "Select an Operator",
        },
    );
    let (selected_field, set_selected_field) = create_signal(
        cx,
        Item {
            id: "default",
            value: "Select a Field",
        },
    );
    let InputUpdater { set_input_objects } = use_context(cx).unwrap();

    create_effect(cx, move |_| {
        writer.update(move |input| input.comparison_op = selected_comp_op.get().id.to_string());
        writer.update(move |input| input.field = selected_field.get().id.to_string());
    });

    view! { cx,
        <div class="input-row" id=id>
           <Dropdown
                items=fields
                selected=selected_field
                set_selected=set_selected_field
            />
            <Dropdown
                items=comp_ops
                selected=selected_comp_op
                set_selected=set_selected_comp_op
            />
            <input
                class="input input-bordered input-accent w-full"
                type="text"
                on:input=move |ev| {
                    writer.update(move |input| input.value = event_target_value(&ev))
                }
            />

            <button class="btn btn-error" on:click=move |_| {
                set_input_objects
                    .update(move |inputs| inputs.retain(|(input_id, _)| input_id != &id))
            }>"x"</button>
        </div>
    }
}

#[component]
pub fn OutputArea(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="output-area overflow-x-auto overflow-y-auto">
            <OutputTable />
        </div>
    }
}

#[component]
pub fn OutputTable(cx: Scope) -> impl IntoView {
    let Fields { fields } = use_context(cx).unwrap();
    let QueryOutput { value } = use_context(cx).unwrap();
    let unwrap_data = move || match value.get() {
        Some(wrapped_data) => match wrapped_data {
            Ok(data) => data,
            Err(_) => Vec::new(),
        },
        None => Vec::new(),
    };

    view! {
        cx,
        <table class="output-table table">
            <tr>
                <For
                    each=fields
                    key=|field| field.value
                    view=move |cx, field| {
                    view! {
                        cx,
                        <th>{move || { field.value }}</th>
                    }
                }
                />
            </tr>
            <For
                each=unwrap_data
                key=|result| result.to_owned()
                view=move |cx, result| {
            view! {
                cx,
                <SummaryRow data=result />
            }
        }
        />
        </table>
    }
}

#[component]
pub fn SummaryRow(cx: Scope, data: Data) -> impl IntoView {
    view! { cx,
        <tr class="summary-row hover">
            <td><a href=data.caltech_href>{data.pl_name}</a></td>
            <td>{data.hostname}</td>
            <td>{data.pl_letter}</td>
            <td>{data.sy_snum}</td>
            <td>{data.sy_pnum}</td>
            <td>{data.sy_mnum}</td>
            <td>{data.cb_flag}</td>
            <td>{data.discovery_method}</td>
            <td>{data.disc_year}</td>
            <td><a href=data.disc_refhref>{data.disc_refname}</a></td>
            <td>{data.disc_pubdate}</td>
        </tr>
    }
}

#[component]
pub fn DetailRow(
    _cx: Scope,
    _data: HashMap<String, String>,
    _id: usize,
    _keys: Vec<String>,
) -> impl IntoView {
    todo!()
}

#[component]
pub fn Dropdown(
    cx: Scope,
    items: ReadSignal<Vec<Item>>,
    selected: ReadSignal<Item>,
    set_selected: WriteSignal<Item>,
) -> impl IntoView {
    let (open, set_open) = create_signal(cx, false);
    view! { cx,
        <details class="dropdown mb-32" value=move || selected().id.to_string() prop:open=open>
            <summary class="btn m-1">
                {move || selected().value.to_string()}
            </summary>
                <ul class="dropdown-content z-[1] menu shadow bg-base-100 rounded-box w-52">
                    <For
                        each=items
                        key=|item| item.id
                        view=move |cx, item| {
                            view! { cx,
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
