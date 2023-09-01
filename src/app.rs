use crate::{
    api::QueryDb,
    model::{data::Data, input::Input},
};
use bson::Document;
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
            id: "hd_name",
            value: "HD ID",
        },
        Item {
            id: "hip_name",
            value: "HIP Name",
        },
        Item {
            id: "tic_id",
            value: "TIC ID",
        },
        Item {
            id: "gaia_id",
            value: "GAIA ID",
        },
        Item {
            id: "default_flag",
            value: "Default Parameter Set",
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
                <button class="btn btn-outline btn-accent" on:click=clear_input>"Clear Input"</button>
            </div>
            <For
                each=input_objects
                key=|input_objects| input_objects.0
                view=move |cx, (id, (_, ws))| {
                    view! { cx, <InputRow id=id writer=ws/> }
                }
            />

            <div class="submit-area">
                <button class="btn btn-primary" on:click=submit_handler>"Submit"</button>
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

    let initial_log_ops = vec![
        Item {
            id: "and",
            value: "AND",
        },
        Item {
            id: "or",
            value: "OR",
        },
    ];

    let Fields { fields } = use_context(cx).unwrap();

    let (log_ops, _set_log_ops) = create_signal(cx, initial_log_ops);

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
    let (selected_log_op, set_selected_log_op) = create_signal(
        cx,
        Item {
            id: "defualt",
            value: "Select an Operator",
        },
    );
    let InputUpdater { set_input_objects } = use_context(cx).unwrap();

    create_effect(cx, move |_| {
        writer.update(move |input| input.logical_op = selected_log_op.get().id.to_string());
        writer.update(move |input| input.comparison_op = selected_comp_op.get().id.to_string());
        writer.update(move |input| input.field = selected_field.get().id.to_string());
    });

    view! { cx,
        <div class="input-row" id=id>
            <Dropdown
                items=log_ops
                selected=selected_log_op
                set_selected=set_selected_log_op
                fallback=|_| ()
            />
            <Dropdown
                items=fields
                selected=selected_field
                set_selected=set_selected_field
                fallback=|_cx| ()
            />
            <Dropdown
                items=comp_ops
                selected=selected_comp_op
                set_selected=set_selected_comp_op
                fallback=|_cx| ()
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
        <div class="output-area overflow-x-auto">
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
            <td>{data.pl_name}</td>
            <td>{data.hostname}</td>
            <td>{data.pl_letter}</td>
            <td>{data.hd_name}</td>
            <td>{data.hip_name}</td>
            <td>{data.tic_id}</td>
            <td>{data.gaia_id}</td>
            <td>{data.default_flag}</td>
            <td>{data.sy_snum}</td>
            <td>{data.sy_pnum}</td>
            <td>{data.sy_mnum}</td>
            <td>{data.cb_flag}</td>
            <td>{data.discovery_method}</td>
            <td>{data.disc_year}</td>
            <td>{data.disc_refname}</td>
            <td>{data.disc_pubdate}</td>
        </tr>
    }
}

#[component]
pub fn DetailRow(
    cx: Scope,
    data: HashMap<String, String>,
    id: usize,
    keys: Vec<String>,
) -> impl IntoView {
    todo!()
}

#[component]
pub fn Dropdown<F, IV>(
    cx: Scope,
    items: ReadSignal<Vec<Item>>,
    selected: ReadSignal<Item>,
    set_selected: WriteSignal<Item>,
    fallback: F,
) -> impl IntoView
where
    F: Fn(Scope) -> IV + 'static,
    IV: IntoView,
{
    let (expand, set_expand) = create_signal(cx, false);

    let expand_event = move |_| {
        if expand() == true {
            set_expand.set(false);
        } else {
            set_expand.set(true);
        }
    };

    let fallback = store_value(cx, fallback);

    view! { cx,
        <div class="drop-down" value=move || selected().id.to_string()>
            <button class="drop-down" on:click=expand_event>
                {move || selected().value.to_string()}
            </button>
            <Show when=expand fallback=move |cx| fallback.with_value(|fallback| fallback(cx))>
                <ul class="option-list">
                    <For
                        each=items
                        key=|item| item.id
                        view=move |cx, item| {
                            view! { cx,
                                <li
                                    value=item.id
                                    on:click=move |_| {
                                        set_selected.set(item);
                                        set_expand.set(false);
                                    }
                                >
                                    {move || item.value}
                                </li>
                            }
                        }
                    />

                </ul>
            </Show>
        </div>
    }
}
