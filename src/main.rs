#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use tab_select::TabVariant;

mod tab_select {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct TabVariant {
        pub name: String,
        pub variant: Element,
    }

    #[derive(Props, Clone, PartialEq)]
    pub struct TabSelectProps {
        tabs: Vec<TabVariant>,
        #[props(default)]
        pub class: String,
    }

    pub fn TabSelect(TabSelectProps { tabs, class }: TabSelectProps) -> Element {
        let mut selected_tab = use_signal::<usize>(Default::default);
        let tab = |idx: usize, name: &str| {
            let selected = selected_tab.read().eq(&idx);
            rsx! {
                button {
                    style: if selected {"border: 3px solid red"} else {""},
                    onclick: move |_| selected_tab.set(idx),
                    "{name}"
                }
            }
        };
        rsx! {
            div { class: "TabSelect {class}", style: "--tab-count: {tabs.len()}",
                {
                    tabs
                    .iter()
                    .enumerate()
                    .map(|(idx, TabVariant { name,  .. })| tab(idx, name))
                }
                div {
                    class: "contents",
                    {
                        let idx = selected_tab.read();
                        tabs
                            .get(*idx)
                            .map(|TabVariant { variant, .. }| {
                                variant
                            })

                    }
                }
            }
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let link = rsx! {Link {
        to: Route::Blog {
            id: count()
        },
        "Go to blog"
    }};
    let high_five_counter_1 = rsx! {
        div {
            h1 { "High-Five counter 1: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    };
    let high_five_counter_2 = rsx! {
        div {
            h1 { "High-Five counter 2: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    };
    let tabs = [link, high_five_counter_1, high_five_counter_2]
        .into_iter()
        .enumerate()
        .map(|(idx, variant)| TabVariant {
            name: format!("Tab 1{idx}"),
            variant,
        })
        .collect::<Vec<_>>();
    rsx! {
        tab_select::TabSelect {
            tabs: tabs,
        }
    }
}
