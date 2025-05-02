use crate::yew::context::SidebarContext;
use crate::yew::context::use_sidebar;
use web_sys::{UrlSearchParams, wasm_bindgen::JsValue, window};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MenuItemProps {
    #[prop_or_default]
    pub label: &'static str,
    #[prop_or_default]
    pub href: &'static str,
    #[prop_or_default]
    pub icon_html: Html,
    #[prop_or_default]
    pub badge: Option<&'static str>,

    // General styling
    #[prop_or("text-decoration: none;")]
    pub anchor_style: &'static str,
    #[prop_or_default]
    pub anchor_class: &'static str,

    #[prop_or("display: flex; align-items: center; gap: 10px; padding: 8px; border-radius: 6px;")]
    pub item_style: &'static str,
    #[prop_or_default]
    pub item_class: &'static str,

    // Collapsed layout style
    #[prop_or(
        "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 8px; border-radius: 6px;"
    )]
    pub collapsed_style: &'static str,
    #[prop_or("font-size: 10px; text-align: center;")]
    pub collapsed_label_style: &'static str,

    // Selected item style
    #[prop_or("background-color: #1e293b; color: white;")]
    pub selected_style: &'static str,

    // Badge style
    #[prop_or(
        "background: red; color: white; padding: 2px 6px; border-radius: 12px; font-size: 10px;"
    )]
    pub badge_style: &'static str,

    pub selected: UseStateHandle<String>,
}

#[function_component(MenuItem)]
pub fn menu_item(props: &MenuItemProps) -> Html {
    let SidebarContext(ctx) = use_sidebar();

    let location = window().unwrap().location();
    let search = location.search();
    let url_params = UrlSearchParams::new_with_str(&search.unwrap_or_default()).unwrap();
    let selected_from_url = url_params.get("selected").unwrap_or_default();

    let is_selected = *props.selected == props.label || selected_from_url == props.label;
    let final_style = format!(
        "{} {}",
        if ctx.is_collapsed {
            props.collapsed_style
        } else {
            props.item_style
        },
        if is_selected {
            props.selected_style
        } else {
            ""
        }
    );

    let on_click = {
        let label = props.label.to_string();
        let selected = props.selected.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let label = label.clone();
            if let Some(win) = window() {
                if let Ok(url) = web_sys::Url::new(&win.location().href().unwrap()) {
                    let params = UrlSearchParams::new_with_str(url.search().as_str()).unwrap();
                    params.set("selected", &label);
                    url.set_search(&params.to_string().as_string().unwrap());
                    win.history()
                        .unwrap()
                        .push_state_with_url(&JsValue::NULL, "", Some(&url.href()))
                        .ok();
                }
            }
            selected.set(label);
        })
    };

    let item_layout = if ctx.is_collapsed {
        html! {
            <div
                class={props.item_class}
                style={final_style}
                aria-label={props.label}
            >
                { props.icon_html.clone() }
                <span style={props.collapsed_label_style}>{ props.label }</span>
            </div>
        }
    } else {
        html! {
            <div
                class={props.item_class}
                style={final_style}
                aria-label={props.label}
            >
                { props.icon_html.clone() }
                <span>{ props.label }</span>
                {
                    if let Some(badge) = props.badge {
                        html! {
                            <span style={props.badge_style}>{ badge }</span>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    };

    html! {
        <a
            class={props.anchor_class}
            style={props.anchor_style}
            href={props.href}
            onclick={on_click}
            aria-current={if is_selected { Some("page") } else { None }}
        >
            { item_layout }
        </a>
    }
}
