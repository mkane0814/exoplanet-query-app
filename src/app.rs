use crate::model::input::{Input, Query};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::collections::HashMap;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    let (query, set_query) = create_signal(cx, Query::new());

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <div class="bg"></div>
        // sets the document title
        <Title text="Welcome to stuff!"/>
        <InputArea/>
    }
}

type InputHolder = Vec<(usize, (ReadSignal<Input>, WriteSignal<Input>))>;

#[derive(Clone, Copy)]
struct InputUpdater {
    set_input_objects: WriteSignal<InputHolder>,
}

#[derive(Clone, Copy)]
pub struct Item {
    pub id: &'static str,
    pub value: &'static str,
}

#[component]
pub fn InputArea(cx: Scope) -> impl IntoView {
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
        for (_id, (rs, _ws)) in input_objects.get() {
            leptos::log!("{:?}", rs.get())
        }
    };

    view! { cx,
        <div class="input-area">
            <div class="input-controls">
                <button on:click=add_input>"Add Input"</button>
                <button on:click=clear_input>"Clear Input"</button>
            </div>
            <For
                each=input_objects
                key=|input_objects| input_objects.0
                view=move |cx, (id, (rs, ws))| {
                    view! { cx, <InputRow id=id reader=rs writer=ws/> }
                }
            />

            <div class="submit-area">
                <button on:click=submit_handler>"Submit"</button>
            </div>
        </div>
    }
}

#[component]
pub fn InputRow(
    cx: Scope,
    id: usize,
    reader: ReadSignal<Input>,
    writer: WriteSignal<Input>,
) -> impl IntoView {
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
            id: "discoverymethod",
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

    let initial_comp_ops = vec![
        Item {
            id: "equals",
            value: "=",
        },
        Item {
            id: "not-equals",
            value: "!=",
        },
        Item {
            id: "less-than",
            value: "<",
        },
        Item {
            id: "greater-than",
            value: ">",
        },
        Item {
            id: "greater-than-or-equals",
            value: ">=",
        },
        Item {
            id: "less-than-or-equals",
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

    let (log_ops, set_log_ops) = create_signal(cx, initial_log_ops);

    let (comp_ops, set_comp_ops) = create_signal(cx, initial_comp_ops);

    let (fields, set_fields) = create_signal(cx, initial_fields);

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
                fallback=|cx| ()
            />
            <Dropdown
                items=comp_ops
                selected=selected_comp_op
                set_selected=set_selected_comp_op
                fallback=|cx| ()
            />
            <input
                type="text"
                on:input=move |ev| {
                    writer.update(move |input| input.value = event_target_value(&ev))
                }
            />

            <button on:click=move |_| {
                set_input_objects
                    .update(move |inputs| inputs.retain(|(input_id, _)| input_id != &id))
            }>"x"</button>
        </div>
    }
}

#[component]
pub fn SummaryRow(
    cx: Scope,
    data: ReadSignal<HashMap<&'static str, &'static str>>,
    id: usize,
    keys: ReadSignal<Vec<&'static str>>,
) -> impl IntoView {
    view! { cx,
        <div class="summary-row" id=id>
            <For
                each=keys
                key=|id| *id
                view=move |cx, id| {
                    view! { cx,
                        <div class="summary-row-cell">
                            {move || {
                                    if let Some(value) = data().get(id) {
                                        value
                                    } else {
                                        ""
                                    }
                                }
                            }
                        </div>
                    }

                }
            />

        </div>
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
