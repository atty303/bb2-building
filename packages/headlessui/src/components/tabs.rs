use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;

use crate::components::RenderFn;

#[derive(Clone)]
struct TabState {
    selected: Signal<usize>,
}

pub struct TabGroupRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
    pub selected: usize,
}

/// The main Tab.Group component.
#[component]
pub fn TabGroup(
    #[props(default = 0)] default_index: usize,
    selected_index: Option<usize>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    render: Option<RenderFn<TabGroupRenderArgs>>,
    children: Element,
) -> Element {
    let state = use_signal(|| TabState {
        selected: Signal::new(selected_index.unwrap_or(default_index)),
    });
    let _ = use_context_provider(|| state());

    if let Some(render) = render {
        render.call(TabGroupRenderArgs {
            attrs: attributes,
            children,
            selected: (state.read().selected)(),
        })
    } else {
        rsx! {
            div {
                ..attributes,
                {children}
            }
        }
    }
}

pub struct TabListRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
    pub selected: usize,
}

#[component]
pub fn TabList(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    render: Option<RenderFn<TabListRenderArgs>>,
    children: Element,
) -> Element {
    let state = try_use_context::<TabState>().expect("TabList must be a child of TabGroup");
    let mut attrs = vec![Attribute::new(
        "role",
        AttributeValue::Text("tablist".to_string()),
        None,
        false,
    )];
    attrs.extend(attributes);
    attrs.sort_by_key(|a| a.name);

    if let Some(render) = render {
        render.call(TabListRenderArgs {
            attrs,
            children,
            selected: (state.selected)(),
        })
    } else {
        rsx! {
            div {
                ..attrs,
                {children}
            }
        }
    }
}

pub struct TabRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
    pub selected: bool,
}

#[component]
pub fn Tab(
    index: usize,
    #[props(default = false)] _disabled: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    render: Option<RenderFn<TabRenderArgs>>,
    children: Element,
) -> Element {
    let mut state = try_use_context::<TabState>().expect("Tab must be a child of TabGroup");
    let selected = state.selected == index;

    let mut attrs = vec![
        Attribute::new("role", AttributeValue::Text("tab".to_string()), None, false),
        Attribute::new(
            "tabindex",
            AttributeValue::Text(if selected {
                "0".to_string()
            } else {
                "-1".to_string()
            }),
            None,
            false,
        ),
        Attribute::new(
            "onclick",
            AttributeValue::listener({
                move |_: Event<PlatformEventData>| {
                    *state.selected.write() = index;
                }
            }),
            None,
            false,
        ),
    ];
    attrs.extend(attributes);
    attrs.sort_by_key(|a| a.name);

    if let Some(render) = render {
        render.call(TabRenderArgs {
            attrs,
            children,
            selected,
        })
    } else {
        rsx! {
            a {
                ..attrs,
                {children}
            }
        }
    }
}

pub struct TabPanelsRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
    pub selected: usize,
}

#[component]
pub fn TabPanels(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    render: Option<RenderFn<TabPanelsRenderArgs>>,
    children: Element,
) -> Element {
    let state = try_use_context::<TabState>().expect("TabPanels must be a child of TabGroup");

    if let Some(render) = render {
        render.call(TabPanelsRenderArgs {
            attrs: attributes,
            children,
            selected: (state.selected)(),
        })
    } else {
        rsx! {
            div {
                ..attributes,
                {children}
            }
        }
    }
}

pub struct TabPanelRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
    pub selected: bool,
}

#[component]
pub fn TabPanel(
    index: usize,
    #[props(default = false)] r#static: bool,
    #[props(default = true)] unmount: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    render: Option<RenderFn<TabPanelRenderArgs>>,
    children: Element,
) -> Element {
    let state = try_use_context::<TabState>().expect("TabPanel must be a child of TabGroup");
    let selected = state.selected == index;

    if r#static || (unmount && selected) {
        let mut attrs = vec![
            Attribute::new(
                "role",
                AttributeValue::Text("tabpanel".to_string()),
                None,
                false,
            ),
            Attribute::new(
                "tabindex",
                AttributeValue::Text(if selected {
                    "0".to_string()
                } else {
                    "-1".to_string()
                }),
                None,
                false,
            ),
        ];
        attrs.extend(attributes);
        attrs.sort_by_key(|a| a.name);

        if let Some(render) = render {
            render.call(TabPanelRenderArgs {
                attrs,
                children,
                selected,
            })
        } else {
            rsx! {
                div {
                    ..attrs,
                    {children}
                }
            }
        }
    } else {
        None
    }
}
