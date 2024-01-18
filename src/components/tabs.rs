use dioxus::core::AttributeValue;
use dioxus::hooks::error::UseSharedStateResult;
use dioxus::prelude::*;
use std::any::TypeId;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;

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

// #[component]
// pub fn Tab<'a, F>(
//     cx: Scope<'a>,
//     index: usize,
//     #[props(default = false)] disabled: bool,
//     render: F,
// ) -> Element<'a>
// where
//     F: FnOnce(Vec<Attribute<'a>>, bool) -> Element,
// {
//     let state = use_shared_state::<TabState>(cx).expect("Tab must be a child of TabGroup");
//     let selected = state.read().selected == *index;
//
//     let mut attrs = vec![];
//     attrs.push(Attribute::new(
//         "role",
//         AttributeValue::Text("tab"),
//         None,
//         false,
//     ));
//     render(attrs, selected)
// }

#[component]
fn TabProps<'a>(cx: Scope, role: &'a str) -> Element {
    render! {
        div {
            role: "tab",
        }
    }
}

#[derive(Debug)]
pub struct Hoge<'a> {
    phantom: PhantomData<&'a ()>,
}

#[derive(Props, PartialEq)]
pub struct TabProps<'a, F>
where
    F: Fn(&'a Vec<Attribute<'a>>, bool) -> Element<'a>,
{
    index: usize,
    #[props(default = false)]
    disabled: bool,
    render: F,
    #[props(default = PhantomData)]
    _phantom: PhantomData<&'a ()>,
}

#[allow(non_snake_case)]
pub fn Tab<'a, F>(cx: Scope<'a, TabProps<'a, F>>) -> Element<'a>
where
    F: Fn(&'a Vec<Attribute<'a>>, bool) -> Element<'a>,
{
    let state = use_shared_state::<TabState>(cx).expect("Tab must be a child of TabGroup");
    let selected = state.read().selected == cx.props.index;

    let attrs = cx.bump().alloc(vec![
        // Attribute::new("role", AttributeValue::Text("tab"), None, false),
        // Attribute::new(
        //     "tabindex",
        //     AttributeValue::Text(if selected { "0" } else { "-1" }),
        //     None,
        //     true,
        // ),
        Attribute::new(
            "onclick",
            cx.listener(move |e: Event<PlatformEventData>| {
                log::debug!("onclick: {}", cx.props.index);
                state.write().selected = cx.props.index;
                log::debug!("{:?}", state.read().selected);
            }),
            None,
            false,
        ),
    ]);
    // let attr = Attribute::new("role", AttributeValue::Text("tab").into(), None, false);
    // attrs.push(attr);

    (cx.props.render)(attrs, selected)
    // render! {
    //     button {
    //         ..*attrs,
    //         "B"
    //     }
    // }
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
    // let selected = state.read().selected == *index;
    // if *r#static || (*unmount && selected) {
    //     render! {
    //         div {
    //             role: "tabpanel",
    //             tabindex: if selected { 0 } else { -1 },
    //             {children}
    //         }
    //     }
    // } else {
    None
    // }
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
