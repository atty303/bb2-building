use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;

struct DialogState {
    open: bool,
}

pub struct RenderFn<T = ()> {
    pub(super) callback: Rc<RefCell<Option<RenderCallback<T>>>>,
}

impl<T> Clone for RenderFn<T> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback.clone(),
        }
    }
}

impl<T> PartialEq for RenderFn<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.callback, &other.callback)
    }
}

impl<T> RenderFn<T> {
    pub fn new(mut f: impl FnMut(T) -> Element + 'static) -> RenderFn<T> {
        let callback = Rc::new(RefCell::new(Some(
            Box::new(move |event: T| f(event)) as Box<dyn FnMut(T) -> Element>
        )));
        RenderFn { callback }
    }

    pub fn call(&self, event: T) -> Element {
        if let Some(callback) = self.callback.borrow_mut().as_mut() {
            callback(event)
        } else {
            None
        }
    }
}

type RenderCallback<T> = Box<dyn FnMut(T) -> Element>;

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
    on_close: EventHandler<()>,
    render: Option<RenderFn<DialogRenderArgs>>,
    children: Element,
) -> Element {
    let state = use_signal(|| DialogState { open });
    let _ = use_context_provider(|| state);

    let mut attrs = Vec::new();

    if let Some(render) = render {
        render.call(DialogRenderArgs {
            attrs,
            children,
            open,
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

pub struct DialogPanelRenderArgs {
    pub attrs: Vec<Attribute>,
    pub children: Element,
}

/// This indicates the panel of your actual Dialog. Clicking outside of this component will trigger the onClose of the Dialog component.
#[component]
pub fn DialogPanel(render: Option<RenderFn<DialogPanelRenderArgs>>, children: Element) -> Element {
    let state = use_context::<Signal<DialogState>>();

    let mut attrs = Vec::new();

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
