#![allow(non_snake_case)]

use dioxus::core::DynamicNode;
use dioxus::prelude::*;
use dioxus_signals::{use_signal, Signal};
use dioxus_web::WebEventExt;
use wasm_bindgen::JsCast;

pub fn use_modal<P: 'static, T: 'static>(cx: &ScopeState, box_class: String) -> &UseModal<P, T> {
    let modal_ref: &UseRef<Option<web_sys::HtmlDialogElement>> = use_ref(cx, || None);
    let props = use_signal(cx, || None::<Signal<P>>);
    let done = use_ref(cx, || None);

    cx.use_hook(move || UseModal {
        modal_ref: modal_ref.clone(),
        box_class: box_class.clone(),
        props: props.clone(),
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
                    class: "modal-box {cx.props.box_class}",
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

pub struct UseModal<P: 'static, T: 'static> {
    pub modal_ref: UseRef<Option<web_sys::HtmlDialogElement>>,
    pub box_class: String,
    pub props: Signal<Option<Signal<P>>>,
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
pub struct ModalDialogProps<'a, P: 'static, T: 'static> {
    pub props: Signal<Option<Signal<P>>>,
    pub on_result: EventHandler<'a, T>,
}

impl<P, T> UseModal<P, T> {
    pub fn component<'a>(
        &self,
        cx: &'a ScopeState,
        child: fn(Scope<'a, ModalDialogProps<'a, P, T>>) -> Element<'a>,
    ) -> DynamicNode<'a> {
        let modal_ref = self.modal_ref.clone();
        let done = self.done.clone();

        let child_component = cx.component(
            child,
            ModalDialogProps {
                props: self.props,
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

    pub fn show_modal(&self, props: Signal<P>, done: impl Fn(T) + 'static) {
        *self.done.write() = Some(Box::new(done));
        *self.props.write() = Some(props.clone());

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
