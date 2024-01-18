use std::marker::PhantomData;

use dioxus::core::{AttributeValue, DynamicNode};
use dioxus::prelude::*;

struct TabState {
    selected: usize,
}

/// The main Tab.Group component.
#[component]
pub fn TabGroup<'a>(
    cx: Scope<'a>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute<'a>>,
    #[props(default = 0)] default_index: usize,
    selected_index: Option<usize>,
    children: Element<'a>,
) -> Element<'a> {
    use_shared_state_provider(cx, || TabState {
        selected: selected_index.unwrap_or(*default_index),
    });

    render! {
        div {
            ..*attributes,
            {children}
        }
    }
}

#[component]
pub fn TabList<'a>(
    cx: Scope<'a>,
    selected_index: Option<usize>,
    children: Element<'a>,
) -> Element<'a> {
    render! {
        div {
            role: "tablist",
            {children}
        }
    }
}

// pub type RenderTab<'a> = dyn Fn(&'a Vec<Attribute<'a>>, &'a Element<'a>, bool) -> Element<'a>;

#[component]
pub fn Tab<'a, F: Fn(&'a Vec<Attribute<'a>>, &'a Element<'a>, bool) -> Element<'a>>(
    cx: Scope<'a>,
    index: usize,
    #[props(default = false)] disabled: bool,
    render: Option<Box<F>>,
    children: Element<'a>,
    #[props(default = PhantomData)] _phantom: PhantomData<&'a F>,
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

    if let Some(r) = render {
        r(attrs, children, selected)
    } else {
        render! {
            button {
                ..*attrs,
                {children}
            }
        }
    }
}

#[component]
pub fn TabPanels<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    render! {
        {children}
    }
}

#[component]
pub fn TabPanel<'a>(
    cx: Scope<'a>,
    index: usize,
    #[props(default = false)] r#static: bool,
    #[props(default = true)] unmount: bool,
    children: Element<'a>,
) -> Element<'a> {
    let state = use_shared_state::<TabState>(cx).expect("TabPanel must be a child of TabGroup");
    let selected = state.read().selected == *index;
    if *r#static || (*unmount && selected) {
        render! {
            div {
                role: "tabpanel",
                tabindex: if selected { 0 } else { -1 },
                {children}
            }
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[component]
    fn Example(cx: Scope) -> Element {
        render! {
            // TabGroup {
            //     TabList {
            //         // Tab { index: 0, "Tab 1" }
            //         // Tab { index: 1, "Tab 2" }
            //         // Tab { index: 2, "Tab 3" }
            //     }
            //     TabPanels {
            //         TabPanel { index: 0, "Panel 1" }
            //         TabPanel { index: 1, "Panel 2" }
            //         TabPanel { index: 2, "Panel 3" }
            //     }
            // }
            ""
        }
    }
}
