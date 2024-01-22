#![allow(non_snake_case)]

use dioxus::core::DynamicNode;
use dioxus::prelude::*;
use dioxus_web::WebEventExt;
use wasm_bindgen::JsCast;

pub fn use_modal<T: 'static>(cx: &ScopeState, box_class: String) -> &UseModal<T> {
    let modal_ref: &UseRef<Option<web_sys::HtmlDialogElement>> = use_ref(cx, || None);
    let done = use_ref(cx, || None);

    cx.use_hook(move || UseModal {
        modal_ref: modal_ref.clone(),
        box_class: box_class.clone(),
        done: done.clone(),
        component: |cx| render! {
            dialog {
                class: "modal backdrop:backdrop-blur",
                onmounted: move |e| {
                    let el = e
                    .web_event()
                        // .get_raw_element().expect("expecting raw element")
                        // .downcast_ref::<web_sys::Element>().expect("expecting Element")
                        .dyn_ref::<web_sys::HtmlDialogElement>().expect("expecting HtmlDialogElement");

                    cx.props.modal_ref.write().replace(el.clone());
                },
                div {
                    class: "modal-box relative {cx.props.box_class}",
                    button {
                        class: "btn btn-xs btn-circle btn-ghost absolute right-1 top-1",
                        tabindex: -1,
                        onclick: move |_| {
                            if let Some(el) = cx.props.modal_ref.read().as_ref() {
                                el.close();
                            };
                        },
                        // OutlineIcon {
                        //     icon: Shape::XMark,
                        // }
                    }
                    {&cx.props.children}
                }
                form {
                    class: "modal-backdrop",
                    method: "dialog",
                    button { "close" }
                }
            }
        },
    })
}

pub struct UseModal<T: 'static> {
    pub modal_ref: UseRef<Option<web_sys::HtmlDialogElement>>,
    pub box_class: String,
    pub done: UseRef<Option<Box<dyn Fn(T)>>>,
    pub component: for<'a> fn(Scope<'a, ModalProps<'a>>) -> Element<'a>,
}

#[derive(Props, Clone)]
pub struct ModalProps<'a> {
    pub modal_ref: UseRef<Option<web_sys::HtmlDialogElement>>,
    pub box_class: String,
    pub children: Element<'a>,
}

#[derive(Props)]
pub struct ModalDialogProps<'a, T: 'static> {
    pub on_result: EventHandler<'a, T>,
}

impl<T> UseModal<T> {
    pub fn component<'a>(
        &self,
        cx: &'a ScopeState,
        child: fn(Scope<'a, ModalDialogProps<'a, T>>) -> Element<'a>,
    ) -> DynamicNode<'a> {
        let modal_ref = self.modal_ref.clone();
        let done = self.done.clone();

        let child_component = cx.component(
            child,
            ModalDialogProps {
                on_result: cx.event_handler(move |e| {
                    if let Some(d) = done.read().as_ref() {
                        d(e);
                    }
                    if let Some(el) = modal_ref.read().as_ref() {
                        el.close();
                    };
                }),
            },
            "ModalDialog",
        );
        cx.component(
            self.component,
            ModalProps {
                modal_ref: self.modal_ref.clone(),
                box_class: self.box_class.clone(),
                children: render! { {child_component} },
            },
            "Modal",
        )
    }

    pub fn show_modal(&self, done: impl Fn(T) + 'static) {
        *self.done.write() = Some(Box::new(done));

        if let Some(el) = self.modal_ref.read().as_ref() {
            el.show_modal().expect("show_modal failed");
        };
    }

    // pub fn close(&self) {
    //     if let Some(el) = self.modalRef.read().as_ref() {
    //         el.close();
    //     };
    // }
}
