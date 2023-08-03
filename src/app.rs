use crate::model::input::{Input, Query};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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

#[component]
pub fn InputArea(cx: Scope) -> impl IntoView {
    let initial_size = 1;
    let mut next_counter_id = initial_size;

    let initial_inputs = (0..initial_size)
        .map(|id| (id, create_signal(cx, Input::new())))
        .collect::<Vec<_>>();

    let (input_objects, set_input_objects) = create_signal(cx, initial_inputs);

    provide_context(
        cx,
        InputUpdater {
            set_input_objects,
        },
    );

    let add_input = move |_| {
        let sig = create_signal(cx, Input::new());

        set_input_objects.update(move |inputs| inputs.push((next_counter_id, sig)));

        next_counter_id += 1;
    };

    let clear_input = move |_| {
        set_input_objects.update(|inputs| inputs.clear());
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
        ("pl_name", "Planet Name"),
        ("hostname", "Host Name"),
        ("pl_letter", "Planet Letter"),
        ("hd_name", "HD ID"),
        ("hip_name", "HIP Name"),
        ("tic_id", "TIC ID"),
        ("gaia_id", "GAIA ID"),
        ("default_flag", "Default Parameter Set"),
        ("sy_snum", "Number of Stars"),
        ("sy_pnum", "Number of Planets"),
        ("sy_mnum", "Number of Moons"),
        ("cb_flag", "Circumbinary Flag"),
        ("discoverymethod", "Discovery Method"),
        ("disc_year", "Discovery Year"),
        ("disc_refname", "Discovery Reference"),
        ("disc_pubdate", "Discovery Publication Date"),
    ];

    let initial_comp_ops = vec![
        ("equals", "="),
        ("not-equals", "!="),
        ("less-than", "<"),
        ("greater-than", ">"),
        ("greater-than-or-equals", ">="),
        ("less-than-or-equals", "<="),
    ];

    let (comp_ops, set_comp_ops) = create_signal(cx, initial_comp_ops);

    let (fields, set_fields) = create_signal(cx, initial_fields);

    let InputUpdater {
         set_input_objects,
    } = use_context(cx).unwrap();

    let update_field = move |event| { writer.update(move |input| input.field = event_target_value(&event)) }; 

    view! { cx,
        <div class="input-row" id=id field=reader().field>
            <select name="Logical Operators" class="lops">
                <option value="and">"And"</option>
                <option value="or">"Or"</option>
            </select>
            <select on:change=update_field
                name="Fields" class="fields">
                <For
                    each=fields
                    key=|field| field.0
                    view=move |cx, (name, pretty_name)| {
                        view! { cx,
                            <option value=name>
                                {pretty_name}
                            </option>
                        }
                    }
                />

            </select>
            <select name="Comp Ops" class="cops">
                <For
                    each=comp_ops
                    key=|op| op.0
                    view=move |cx, (value, token)| {
                        view! { cx, <option value=value>{token}</option> }
                    }
                />

            </select>
            <input type="text"/>
            <button on:click=move |_| {
                set_input_objects
                    .update(move |inputs| inputs.retain(|(input_id, _)| input_id != &id))
            }>"x"</button>
        </div>
    }
}
