use crate::components::SkillView;
use crate::editor::CodeMirror;
use crate::global::DATABASE;
use crate::ui::{Dialog, SpriteIcon};
use crate::Language;
use classes::classes;
use dioxus::prelude::*;
use dioxus::web::WebEventExt;
use headlessui::{
    RenderFn, Tab, TabGroup, TabList, TabPanel, TabPanelRenderArgs, TabPanels, TabRenderArgs,
};
use markdown_it::plugins::cmark;
use markdown_it::{parser, MarkdownIt};
use wasm_bindgen::closure::Closure;

#[component]
pub fn BuildEditPage(language: Language) -> Element {
    let detail_open = use_signal(|| false);
    let detail_target = use_signal(|| None);

    let mut doc = use_signal(|| {
        "<skill:エンシェントシールド> 強い。 <skill:インビンシブル> <skill:デーモンバイト> ドロップしない？ <skill:採掘> 便利".to_string()
    });

    let mut code_mirror = use_signal(|| None::<CodeMirror>);

    use_effect(move || {
        if let Some(ref cm) = *code_mirror.read() {
            if doc() != cm.value() {
                cm.set_value(doc());
            }
        }
    });

    let rendered = use_memo(move || {
        let md = &mut MarkdownIt::new();
        cmark::add(md);
        let node = md.parse(&doc());
        // tracing::info!("{:#?}", node);
        render_markdown(RenderArgs {
            node: &node,
            open: detail_open.clone(),
            target: detail_target.clone(),
        })
    });

    let render_tab = RenderFn::<TabRenderArgs>::new(move |args| {
        let class = classes!["tab", "tab-active [--tab-bg:oklch(var(--b2))]" => args.selected];
        rsx! {
            a {
                class,
                ..args.attrs,
                {args.children}
            }
        }
    });
    let render_tab_panel = RenderFn::<TabPanelRenderArgs>::new(move |args| {
        let class = classes!["bg-base-200 rounded-b-lg p-4", "hidden" => !args.selected];
        rsx! {
            div {
                class,
                ..args.attrs,
                {args.children}
            }
        }
    });

    rsx! {
        TabGroup {
            TabList {
                class: "tabs-lifted",
                Tab {
                    index: 0,
                    render: Some(render_tab.clone()),
                    "Write"
                }
                Tab {
                    index: 1,
                    render: Some(render_tab.clone()),
                    "Preview"
                }
            }
            TabPanels {
                TabPanel {
                    index: 0,
                    r#static: true,
                    render: Some(render_tab_panel.clone()),
                    div {
                        class: "w-full h-96",
                        onmounted: move |e| {
                            let parent = e.web_event();
                            let on_change = Closure::wrap(
                                Box::new(move |value| {
                                    *doc.write() = value;
                                }) as Box<dyn FnMut(String)>,
                            );
                            let cm = CodeMirror::new(parent, &on_change);
                            on_change.forget();
                            *code_mirror.write() = Some(cm);
                        }
                    }
                }
                TabPanel {
                    index: 1,
                    render: Some(render_tab_panel.clone()),
                    article { class: "prose md:prose-lg max-w-full", {rendered()} }
                }
            }
        }

        DetailDialog { language open: detail_open, target: detail_target }
    }
}

struct RenderArgs<'a> {
    node: &'a markdown_it::Node,
    open: Signal<bool>,
    target: Signal<Option<DetailTarget>>,
}
impl RenderArgs<'_> {
    fn render_children(&self) -> Element {
        rsx! {{self.node.children.iter().map(|child| render_markdown(RenderArgs { node: child, open: self.open, target: self.target }))}}
    }
}

fn render_markdown(mut args: RenderArgs) -> Element {
    let node = &args.node;
    if let Some(_) = node.cast::<parser::core::Root>() {
        args.render_children()
    } else if let Some(n) = node.cast::<parser::inline::Text>() {
        rsx! {"{n.content}"}
    } else if let Some(n) = node.cast::<parser::inline::TextSpecial>() {
        if n.info == "autolink" {
            let content = n.content.split(":").last().unwrap_or(&n.content);
            if let Some(s) = DATABASE().skill.find(content) {
                let t = DetailTarget::Skill(Signal::new(s.clone()));
                rsx! {
                    a {
                        class: "cursor-pointer",
                        onclick: move |_| {
                            *args.open.write() = true;
                            *args.target.write() = Some(t.clone());
                        },
                        prevent_default: "onclick",
                        SpriteIcon {
                            class: "rounded-md align-middle",
                            sprite: Signal::new(s.modes[0].icon.clone()),
                            size: 20
                        }
                        "{content}"
                    }
                }
            } else {
                rsx! {"{n.content}"}
            }
        } else {
            rsx! {"{n.content}"}
        }
    } else if let Some(_) = node.cast::<cmark::block::paragraph::Paragraph>() {
        rsx! {
            p { {args.render_children()} }
        }
    } else if let Some(_) = node.cast::<cmark::block::list::BulletList>() {
        rsx! {
            ul { {args.render_children()} }
        }
    } else if let Some(_) = node.cast::<cmark::block::list::OrderedList>() {
        rsx! {
            ol { {args.render_children()} }
        }
    } else if let Some(_) = node.cast::<cmark::block::list::ListItem>() {
        rsx! {
            li { {args.render_children()} }
        }
    } else if let Some(_) = node.cast::<cmark::inline::newline::Softbreak>() {
        rsx! { br {} }
    } else if let Some(_) = node.cast::<cmark::inline::emphasis::Em>() {
        rsx! {
            em { {args.render_children()} }
        }
    } else if let Some(_) = node.cast::<cmark::inline::emphasis::Strong>() {
        rsx! {
            strong { {args.render_children()} }
        }
    } else if let Some(_) = node.cast::<cmark::inline::backticks::CodeInline>() {
        rsx! {
            code { {args.render_children()} }
        }
    } else if let Some(_) = node.cast::<cmark::inline::autolink::Autolink>() {
        rsx! {{args.render_children()}}
    } else {
        tracing::warn!("Unknown node: {:#?}", node);
        None
    }
}

#[derive(Clone, PartialEq)]
enum DetailTarget {
    Skill(Signal<data::Skill>),
}

#[component]
fn DetailDialog(
    language: Language,
    open: Signal<bool>,
    target: Signal<Option<DetailTarget>>,
) -> Element {
    if let Some(t) = target.read().as_ref() {
        rsx! {
            Dialog {
                open,
                on_close: move |_| {
                    *open.write() = false;
                    *target.write() = None;
                },
                div { class: "mt-12",
                    match t {
                        DetailTarget::Skill(skill) => {
                            rsx! {
                                SkillView { language, skill: *skill }
                            }
                        }
                    }
                }
            }
        }
    } else {
        None
    }
}
