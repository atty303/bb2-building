use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use web_sys::wasm_bindgen::prelude::Closure;
use web_sys::wasm_bindgen::JsCast;

use crate::components::RenderFn;

struct DialogState {
    open: Signal<bool>,
    dialog: Signal<Option<web_sys::HtmlDialogElement>>,
}

pub struct DialogRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
    pub open: bool,
}

/// The main Dialog component.
#[component]
pub fn Dialog(
    /// Whether the Dialog is open or not.
    open: bool,
    /// Called when the Dialog is dismissed (via outside click of the DialogPanel or by pressing the Escape key). Typically used to close the dialog by setting open to false.
    on_close: Option<EventHandler<()>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    render: Option<RenderFn<DialogRenderArgs>>,
    children: Element,
) -> Element {
    let state = use_signal(|| DialogState {
        open: Signal::new(open),
        dialog: Signal::new(None),
    });
    let _ = use_context_provider(|| state);

    use_effect(move || {
        if let Some(dialog) = state.read().dialog.read().as_ref() {
            if *state.read().open.read() {
                if !dialog.has_attribute("open") {
                    dialog.show_modal().unwrap();
                }
            } else {
                if dialog.has_attribute("open") {
                    dialog.close();
                }
            }
        }
    });

    let on_close2 = on_close.clone();
    let func = Box::new(move |e: web_sys::KeyboardEvent| {
        e.prevent_default();
        if e.key() == "Escape" {
            if let Some(dialog) = state.read().dialog.read().as_ref() {
                dialog.close();
            }
            on_close2.as_ref().map(|f| f.call(()));
        }
    }) as Box<dyn FnMut(_)>;

    let handler = Closure::wrap(func);

    let r = use_signal(|| handler);
    use_effect(move || {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .add_event_listener_with_callback("keydown", r.read().as_ref().unchecked_ref())
            .unwrap();
    });

    use_drop(move || {
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .remove_event_listener_with_callback("keydown", r.read().as_ref().unchecked_ref())
            .unwrap();
    });

    let mut attrs = vec![
        Attribute::new(
            "onclick",
            AttributeValue::listener({
                move |event: Event<PlatformEventData>| {
                    event.stop_propagation();
                    on_close.as_ref().map(|f| f.call(()));
                }
            }),
            None,
            false,
        ),
        Attribute::new(
            "onmounted",
            AttributeValue::listener(move |event: Event<PlatformEventData>| {
                let e: MountedEvent = event.map(|e| e.into());
                let el = e
                    .web_event()
                    .dyn_ref::<web_sys::HtmlDialogElement>()
                    .expect("expecting HtmlDialogElement");
                let mut dialog = state.read().dialog;
                *dialog.write() = Some(el.clone());
            }),
            None,
            false,
        ),
        Attribute::new(
            "role",
            AttributeValue::Text("dialog".to_string()),
            None,
            false,
        ),
    ];
    attrs.extend(attributes);
    attrs.sort_by_key(|a| a.name);

    if let Some(render) = render {
        render.call(DialogRenderArgs {
            attrs,
            children,
            open,
        })
    } else {
        rsx! {
            dialog {
                ..attrs,
                {children}
            }
        }
    }
}

pub struct DialogPanelRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
}

/// This indicates the panel of your actual Dialog. Clicking outside of this component will trigger the onClose of the Dialog component.
#[component]
pub fn DialogPanel(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    render: Option<RenderFn<DialogPanelRenderArgs>>,
    children: Element,
) -> Element {
    // let _state = use_context::<Signal<DialogState>>();

    let mut attrs = vec![Attribute::new(
        "onclick",
        AttributeValue::listener({
            move |event: Event<PlatformEventData>| {
                event.stop_propagation();
            }
        }),
        None,
        false,
    )];
    attrs.extend(attributes);
    attrs.sort_by_key(|a| a.name);

    if let Some(render) = render {
        render.call(DialogPanelRenderArgs { attrs, children })
    } else {
        rsx! {
            div {
                ..attrs,
                {children}
            }
        }
    }
}
