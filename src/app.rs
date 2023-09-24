use crate::components::input::Home;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/exoplanet_query_app.css"/>
        <Link rel="icon" type_="image/x-icon" href="/assets/favicon.ico"/>
        <div class="bg-neutral"></div>
        // sets the document title
        <Title text="Exoplanet Query App"/>
        <Router>
            <nav>
                <div class="navbar bg-primary text-primary-content">
                    <A href="/" class="btn btn-ghost normal-case text-xl">
                        "Home"
                    </A>
                </div>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=QueryTabWrapper/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn QueryTabWrapper() -> impl IntoView {
    let tab_initial_id: usize = 1;
    let mut next_tab_id = tab_initial_id;
    let tab_ids = (0..tab_initial_id)
        .map(|id| (id, create_signal(true)))
        .collect::<Vec<_>>();

    let current_active_tab = create_rw_signal(tab_initial_id);

    let (tabs, set_tabs) = create_signal(tab_ids);
    let add_tab = move |_| {
        let sig = create_signal(true);
        set_tabs.update(move |tabs| tabs.push((next_tab_id, sig)));
        current_active_tab.update(move |stored_id| *stored_id = next_tab_id);
        next_tab_id += 1;
    };

    view! {
        <div class="tabs tabs-boxed">
            <For
                each=tabs
                key=|tab| tab.0
                view=move |(id, (_rs, _ws))| {
                    view! {
                        <button
                            id=id
                            class="tab"
                            class:tab-active=move || current_active_tab.get() == id
                            on:click=move |_| {
                                current_active_tab.update(move |stored_id| *stored_id = id)
                            }
                        >

                            {format!("Tab {}", id + 1)}
                        </button>
                        <button
                            id=id
                            class="tab"
                            on:click=move |_| {
                                set_tabs
                                    .update(move |tabs| tabs.retain(|(tab_id, _)| tab_id != &id));
                            }
                        >

                            "x"
                        </button>
                    }
                }
            />

            <button class="tab" on:click=add_tab>
                "+"
            </button>
        </div>
        <For
            each=tabs
            key=|tab| tab.0
            view=move |(id, (_rs, _ws))| {
                view! {
                    <Show when=move || current_active_tab.get() == id fallback=|| ()>
                        <Home id/>
                    </Show>
                }
            }
        />
    }
}
