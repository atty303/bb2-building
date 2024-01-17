use dioxus::prelude::*;
use std::marker::PhantomData;

#[component]
fn Example<'a, F: Fn(&'a Vec<Attribute<'a>>) -> Element<'a>>(
    cx: Scope,
    f: F,
    phantom: Option<PhantomData<&'a ()>>,
) -> Element {
    render! {
        div {
            "Hello, World!"
        }
    }
}

#[component]
fn Test(cx: Scope) -> Element {
    render! {
        Example {
            f: move |attrs| {
                Some({
                                                let __cx = cx;
                                                static TEMPLATE: ::dioxus::core::Template = ::dioxus::core::Template {
                                                    name: "src\\components\\example.rs:17:17:3884",
                                                    roots: &[
                                                        ::dioxus::core::TemplateNode::Element {
                                                            tag: dioxus_elements::div::TAG_NAME,
                                                            namespace: dioxus_elements::div::NAME_SPACE,
                                                            attrs: &[
                                                                ::dioxus::core::TemplateAttribute::Dynamic {
                                                                    id: 0usize,
                                                                },
                                                            ],
                                                            children: &[
                                                                ::dioxus::core::TemplateNode::Text {
                                                                    text: "Hello, World!",
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                    node_paths: &[],
                                                    attr_paths: &[&[0u8]],
                                                };
                                                __cx.vnode(
                                                    None.into(),
                                                    None,
                                                    std::cell::Cell::new(TEMPLATE),
                                                    dioxus::core::exports::bumpalo::collections::Vec::with_capacity_in(
                                                        1usize,
                                                        __cx.bump(),
                                                    )
                                                        .into(),
                                                    __cx.bump().alloc([]),
                                                    __cx.bump().alloc([(*&attrs).into()]),
                                                    // __cx.bump().alloc([(&Vec::<Attribute>::new()).into()]),
                                                )
                                            })
                // render! {
                //     div {
                //         ..attrs,
                //         "Hello, World!"
                //     }
                // }
            }
        }
    }
}
/*
mod example {
    use dioxus::prelude::*;
    ///Properties for the [`Example`] component.
    #[allow(non_camel_case_types)]
    struct ExampleProps<F: Fn(Vec<Attribute<'_>>) -> Element> {
        f: F,
    }
    impl<'__bump, F: Fn(Vec<Attribute<'_>>) -> Element> ExampleProps<F> {
        /**
        Create a builder for building `ExampleProps`.
        On the builder, call `.f(...)` to set the values of the fields.
        Finally, call `.build()` to create the instance of `ExampleProps`.
         */
        #[allow(dead_code, clippy::type_complexity)]
        fn builder(_cx: &'__bump ::dioxus::prelude::ScopeState) -> ExamplePropsBuilder<((),), F> {
            ExamplePropsBuilder {
                fields: ((),),
                _phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    struct ExamplePropsBuilder<TypedBuilderFields, F: Fn(Vec<Attribute<'_>>) -> Element> {
        fields: TypedBuilderFields,
        _phantom: (::core::marker::PhantomData<F>),
    }
    impl<'__bump, F: Fn(Vec<Attribute<'_>>) -> Element> ::dioxus::prelude::Properties<'__bump>
        for ExampleProps<F>
    {
        type Builder = ExamplePropsBuilder<((),), F>;
        const IS_STATIC: bool = false;
        fn builder(_cx: &'__bump ::dioxus::prelude::ScopeState) -> Self::Builder {
            ExampleProps::builder(_cx)
        }
        unsafe fn memoize(&self, other: &Self) -> bool {
            false
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub trait ExamplePropsBuilder_Optional<T> {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
    }
    impl<T> ExamplePropsBuilder_Optional<T> for () {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
            default()
        }
    }
    impl<T> ExamplePropsBuilder_Optional<T> for (T,) {
        fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
            self.0
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<F: Fn(Vec<Attribute<'_>>) -> Element> ExamplePropsBuilder<((),), F> {
        #[allow(clippy::type_complexity)]
        pub fn f(self, f: F) -> ExamplePropsBuilder<((F,),), F> {
            let f = (f,);
            let (_,) = self.fields;
            ExamplePropsBuilder {
                fields: (f,),
                _phantom: self._phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum ExamplePropsBuilder_Error_Repeated_field_f {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<F: Fn(Vec<Attribute<'_>>) -> Element> ExamplePropsBuilder<((F,),), F> {
        #[deprecated(note = "Repeated field f")]
        #[allow(clippy::type_complexity)]
        pub fn f(
            self,
            _: ExamplePropsBuilder_Error_Repeated_field_f,
        ) -> ExamplePropsBuilder<((F,),), F> {
            self
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum ExamplePropsBuilder_Error_Missing_required_field_f {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl<F: Fn(Vec<Attribute<'_>>) -> Element> ExamplePropsBuilder<((),), F> {
        #[deprecated(note = "Missing required field f")]
        pub fn build(
            self,
            _: ExamplePropsBuilder_Error_Missing_required_field_f,
        ) -> ExampleProps<F> {
            {
                #[cold]
                #[track_caller]
                #[inline(never)]
                const fn panic_cold_explicit() -> ! {
                    ::core::panicking::panic_explicit()
                }
                panic_cold_explicit();
            };
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<F: Fn(Vec<Attribute<'_>>) -> Element> ExamplePropsBuilder<((F,),), F> {
        pub fn build(self) -> ExampleProps<F> {
            let (f,) = self.fields;
            let f = f.0;
            ExampleProps { f }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F: Fn(Vec<Attribute<'_>>) -> Element> ::core::marker::StructuralPartialEq for ExampleProps<F> {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl<F: ::core::cmp::PartialEq + Fn(Vec<Attribute<'_>>) -> Element> ::core::cmp::PartialEq
        for ExampleProps<F>
    {
        #[inline]
        fn eq(&self, other: &ExampleProps<F>) -> bool {
            self.f == other.f
        }
    }
    #[allow(non_snake_case)]
    fn Example<'a, F: Fn(Vec<Attribute<'_>>) -> Element>(
        cx: Scope<'a, ExampleProps<F>>,
    ) -> Element {
        #[warn(non_snake_case)]
        #[allow(clippy::inline_always)]
        #[inline(always)]
        fn __dx_inner_comp<'a, F: Fn(Vec<Attribute<'_>>) -> Element>(
            cx: Scope<'a, ExampleProps<F>>,
        ) -> Element {
            let ExampleProps { f } = &cx.props;
            {
                Some({
                    let __cx = cx;
                    static TEMPLATE: ::dioxus::core::Template = ::dioxus::core::Template {
                        name: "src\\components\\example.rs:5:5:3678",
                        roots: &[::dioxus::core::TemplateNode::Element {
                            tag: dioxus_elements::div::TAG_NAME,
                            namespace: dioxus_elements::div::NAME_SPACE,
                            attrs: &[],
                            children: &[::dioxus::core::TemplateNode::Text {
                                text: "Hello, World!",
                            }],
                        }],
                        node_paths: &[],
                        attr_paths: &[],
                    };
                    __cx.vnode(
                        None.into(),
                        None,
                        std::cell::Cell::new(TEMPLATE),
                        dioxus::core::exports::bumpalo::collections::Vec::with_capacity_in(
                            1usize,
                            __cx.bump(),
                        )
                        .into(),
                        __cx.bump().alloc([]),
                        __cx.bump().alloc([]),
                    )
                })
            }
        }
        __dx_inner_comp(cx)
    }
    ///Properties for the [`Test`] component.
    #[allow(non_snake_case)]
    fn Test(cx: Scope) -> Element {
        #[warn(non_snake_case)]
        #[allow(clippy::inline_always)]
        #[inline(always)]
        fn __dx_inner_comp(cx: Scope) -> Element {
            Some({
                let __cx = cx;
                static TEMPLATE: ::dioxus::core::Template = ::dioxus::core::Template {
                    name: "src\\components\\example.rs:14:5:3802",
                    roots: &[::dioxus::core::TemplateNode::Dynamic { id: 0usize }],
                    node_paths: &[&[0u8]],
                    attr_paths: &[],
                };
                __cx.vnode(
                    None.into(),
                    None,
                    std::cell::Cell::new(TEMPLATE),
                    dioxus::core::exports::bumpalo::collections::Vec::with_capacity_in(
                        1usize,
                        __cx.bump(),
                    )
                        .into(),
                    __cx
                        .bump()
                        .alloc([
                            __cx
                                .component(
                                    Example,
                                    fc_to_builder(__cx, Example)
                                        .f(move |attrs| {
                                            Some({
                                                let __cx = cx;
                                                static TEMPLATE: ::dioxus::core::Template = ::dioxus::core::Template {
                                                    name: "src\\components\\example.rs:17:17:3884",
                                                    roots: &[
                                                        ::dioxus::core::TemplateNode::Element {
                                                            tag: dioxus_elements::div::TAG_NAME,
                                                            namespace: dioxus_elements::div::NAME_SPACE,
                                                            attrs: &[
                                                                ::dioxus::core::TemplateAttribute::Dynamic {
                                                                    id: 0usize,
                                                                },
                                                            ],
                                                            children: &[
                                                                ::dioxus::core::TemplateNode::Text {
                                                                    text: "Hello, World!",
                                                                },
                                                            ],
                                                        },
                                                    ],
                                                    node_paths: &[],
                                                    attr_paths: &[&[0u8]],
                                                };
                                                __cx.vnode(
                                                    None.into(),
                                                    None,
                                                    std::cell::Cell::new(TEMPLATE),
                                                    dioxus::core::exports::bumpalo::collections::Vec::with_capacity_in(
                                                        1usize,
                                                        __cx.bump(),
                                                    )
                                                        .into(),
                                                    __cx.bump().alloc([]),
                                                    __cx.bump().alloc([(&attrs).into()]),
                                                )
                                            })
                                        })
                                        .build(),
                                    "Example",
                                ),
                        ]),
                    __cx.bump().alloc([]),
                )
            })
        }
        __dx_inner_comp(cx)
    }
}

*/
