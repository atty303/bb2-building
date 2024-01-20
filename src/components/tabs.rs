#![allow(dead_code)]
use std::marker::PhantomData;

use dioxus::core::AttributeValue;
use dioxus::prelude::*;

struct TabState {
    selected: usize,
}

/// The main Tab.Group component.
#[component]
pub fn TabGroup<'a, F: Fn(&'a Element<'a>, usize) -> Element<'a>>(
    cx: Scope<'a>,
    #[props(default = 0)] default_index: usize,
    selected_index: Option<usize>,
    render: F,
    children: Element<'a>,
) -> Element<'a> {
    use_shared_state_provider(cx, || TabState {
        selected: selected_index.unwrap_or(*default_index),
    });
    let state = use_shared_state::<TabState>(cx).unwrap();

    render(children, state.read().selected)
}

#[component]
pub fn TabList<'a, F: Fn(&'a Vec<Attribute<'a>>, &'a Element<'a>, usize) -> Element<'a>>(
    cx: Scope<'a>,
    render: F,
    children: Element<'a>,
) -> Element<'a> {
    let state = use_shared_state::<TabState>(cx).expect("TabList must be a child of TabGroup");
    let attrs = cx.bump().alloc(vec![Attribute::new(
        "role",
        AttributeValue::Text("tablist"),
        None,
        false,
    )]);

    render(attrs, children, state.read().selected)
}

#[component]
pub fn Tab<'a, F: Fn(&'a Vec<Attribute<'a>>, &'a Element<'a>, bool) -> Element<'a>>(
    cx: Scope<'a>,
    index: usize,
    #[props(default = false)] _disabled: bool,
    render: F,
    children: Element<'a>,
    #[props(default = PhantomData)] _phantom: PhantomData<&'a ()>,
) -> Element<'a> {
    let state = use_shared_state::<TabState>(cx).expect("Tab must be a child of TabGroup");
    let selected = state.read().selected == *index;

    let attrs = cx.bump().alloc(vec![
        Attribute::new("role", AttributeValue::Text("tab"), None, false),
        Attribute::new(
            "tabindex",
            AttributeValue::Text(if selected { "0" } else { "-1" }),
            None,
            false,
        ),
        Attribute::new(
            "onclick",
            cx.listener(move |_: Event<PlatformEventData>| {
                state.write().selected = cx.props.index;
            }),
            None,
            false,
        ),
    ]);

    render(attrs, children, selected)
}

#[component]
pub fn TabPanels<'a, F: Fn(&'a Vec<Attribute<'a>>, &'a Element<'a>, usize) -> Element<'a>>(
    cx: Scope<'a>,
    render: F,
    children: Element<'a>,
) -> Element<'a> {
    let state = use_shared_state::<TabState>(cx).expect("TabPanels must be a child of TabGroup");
    render(cx.bump().alloc(vec![]), children, state.read().selected)
}

#[component]
pub fn TabPanel<'a, F: Fn(&'a Vec<Attribute<'a>>, &'a Element<'a>, bool) -> Element<'a>>(
    cx: Scope<'a>,
    index: usize,
    #[props(default = false)] r#static: bool,
    #[props(default = true)] unmount: bool,
    render: F,
    children: Element<'a>,
) -> Element<'a> {
    let state = use_shared_state::<TabState>(cx).expect("TabPanel must be a child of TabGroup");
    let selected = state.read().selected == *index;

    if *r#static || (*unmount && selected) {
        let attrs = cx.bump().alloc(vec![
            Attribute::new("role", AttributeValue::Text("tabpanel"), None, false),
            Attribute::new(
                "tabindex",
                AttributeValue::Text(if selected { "0" } else { "-1" }),
                None,
                false,
            ),
        ]);

        render(attrs, children, selected)
    } else {
        None
    }
}
