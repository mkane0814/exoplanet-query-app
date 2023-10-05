#![allow(non_snake_case)]

use crate::{
    api::QueryDb,
    components::output::OutputArea,
    model::{
        data::Data,
        input::{Input, PageKind},
    },
};
use leptos::*;

type InputHolder = Vec<(usize, (ReadSignal<Input>, WriteSignal<Input>))>;

#[derive(Clone, Copy)]
pub struct Item {
    pub id: &'static str,
    pub value: &'static str,
    pub fe_id: usize,
}

#[derive(Clone, Copy)]
pub struct Fields {
    pub fields: ReadSignal<Vec<Item>>,
}

#[derive(Clone, Copy)]
pub struct InputUpdater {
    pub set_input_objects: WriteSignal<InputHolder>,
}

#[derive(Clone, Copy)]
pub struct LastId {
    pub last_id: ReadSignal<i64>,
    pub set_last_id: WriteSignal<i64>,
}

#[derive(Clone, Copy)]
pub struct FirstId {
    pub first_id: ReadSignal<i64>,
    pub set_first_id: WriteSignal<i64>,
}

#[derive(Clone, Copy)]
pub struct QueryOutput {
    pub value: ReadSignal<Option<Result<Option<Data>, ServerFnError>>>,
}

#[component]
pub fn Home(id: usize) -> impl IntoView {
    let initial_fields = vec![
        Item {
            id: "pl_name",
            value: "Planet Name",
            fe_id: 0,
        },
        Item {
            id: "hostname",
            value: "Host Name",
            fe_id: 1,
        },
        Item {
            id: "sy_snum",
            value: "Number of Stars",
            fe_id: 2,
        },
        Item {
            id: "sy_pnum",
            value: "Number of Planets",
            fe_id: 3,
        },
        Item {
            id: "sy_mnum",
            value: "Number of Moons",
            fe_id: 4,
        },
        Item {
            id: "cb_flag",
            value: "Circumbinary Flag",
            fe_id: 5,
        },
        Item {
            id: "discovery_method",
            value: "Discovery Method",
            fe_id: 6,
        },
        Item {
            id: "disc_year",
            value: "Discovery Year",
            fe_id: 7,
        },
        Item {
            id: "disc_refname",
            value: "Discovery Reference",
            fe_id: 8,
        },
        Item {
            id: "disc_pubdate",
            value: "Discovery Publication Date",
            fe_id: 9,
        },
    ];

    let (last_id, set_last_id) = create_signal(0);
    let (first_id, set_first_id) = create_signal(0);
    let (fields, _) = create_signal(initial_fields);
    let query_action = create_server_action::<QueryDb>();
    provide_context(LastId {
        last_id,
        set_last_id,
    });
    provide_context(FirstId {
        first_id,
        set_first_id,
    });
    provide_context(Fields { fields });
    provide_context(QueryOutput {
        value: query_action.value().read_only(),
    });
    view! {
        <div id=id>
            <InputArea query_action/>
            <OutputArea/>
        </div>
    }
}

#[component]
pub fn InputArea(
    query_action: Action<QueryDb, Result<Option<Data>, ServerFnError>>,
) -> impl IntoView {
    let initial_size = 1;
    let mut next_counter_id = initial_size;

    let initial_inputs = (0..initial_size)
        .map(|id| (id, create_signal(Input::new())))
        .collect::<Vec<_>>();

    let (input_objects, set_input_objects) = create_signal(initial_inputs);

    provide_context(InputUpdater { set_input_objects });

    let LastId {
        last_id,
        set_last_id: _,
    } = use_context().unwrap();

    let FirstId {
        first_id,
        set_first_id: _,
    } = use_context().unwrap();

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
        query_action.dispatch(QueryDb {
            query: inputs,
            anchor_id: 0i64,
            page_direction: PageKind::Next,
        });
    };

    let next_page = move |_| {
        let mut inputs = Vec::<Input>::new();

        for (_id, (rs, _ws)) in input_objects.get() {
            inputs.push(rs.get());
        }

        query_action.dispatch(QueryDb {
            query: inputs,
            anchor_id: last_id.get(),
            page_direction: PageKind::Next,
        });
    };

    let prev_page = move |_| {
        let mut inputs = Vec::<Input>::new();
        for (_id, (rs, _ws)) in input_objects.get() {
            inputs.push(rs.get());
        }
        query_action.dispatch(QueryDb {
            query: inputs,
            anchor_id: first_id.get(),
            page_direction: PageKind::Prev,
        });
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
                children=move |(id, (_, ws))| {
                    view! { <InputRow id=id writer=ws/> }
                }
            />

            <div class="divider"></div>

            <div class="join grid grid-cols-4 submit-area gap-4">
                <button class="join-item btn btn-outline btn-success" on:click=submit_handler>
                    "Submit"
                </button>
                <button class="join-item btn btn-outline btn-error" on:click=clear_input>
                    "Clear Input"
                </button>
                <button class="join-item btn btn-outline" on:click=prev_page>
                    "Previous Page"
                </button>
                <button class="join-item btn btn-outline" on:click=next_page>
                    "Next"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn InputRow(id: usize, writer: WriteSignal<Input>) -> impl IntoView {
    let initial_comp_ops = vec![
        Item {
            id: "=",
            value: "=",
            fe_id: 0,
        },
        Item {
            id: "!=",
            value: "!=",
            fe_id: 1,
        },
        Item {
            id: "<",
            value: "<",
            fe_id: 2,
        },
        Item {
            id: ">",
            value: ">",
            fe_id: 3,
        },
        Item {
            id: ">=",
            value: ">=",
            fe_id: 4,
        },
        Item {
            id: "<=",
            value: "<=",
            fe_id: 5,
        },
    ];

    let Fields { fields } = use_context().unwrap();

    let (comp_ops, _set_comp_ops) = create_signal(initial_comp_ops);

    let (selected_comp_op, set_selected_comp_op) = create_signal(Item {
        id: "default",
        value: "Select an Operator",
        fe_id: 0,
    });
    let (selected_field, set_selected_field) = create_signal(Item {
        id: "default",
        value: "Select a Field",
        fe_id: 0,
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
                class="input input-bordered input-sm input-info w-full"
                type="text"
                on:input=move |ev| {
                    writer.update(move |input| input.value = event_target_value(&ev))
                }
            />

            <button
                class="btn btn-sm btn-error"
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
pub fn Dropdown(
    items: ReadSignal<Vec<Item>>,
    selected: ReadSignal<Item>,
    set_selected: WriteSignal<Item>,
) -> impl IntoView {
    let (open, set_open) = create_signal(false);
    view! {
        <details class="dropdown" value=move || selected().id.to_string() prop:open=open>
            <summary class="btn btn-outline btn-sm btn-info m-1">
                {move || selected().value.to_string()}
            </summary>
            <ul class="dropdown-content z-[1] menu shadow bg-base-200 rounded-box w-52">
                <For each=items key=|item| item.fe_id let:item>
                    <li
                        value=item.id
                        on:click=move |_| {
                            set_open(false);
                            set_selected(item);
                        }
                    >

                        {move || item.value}
                    </li>
                </For>

            </ul>
        </details>
    }
}
