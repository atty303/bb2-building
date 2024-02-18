use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;

pub mod dialog;

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
