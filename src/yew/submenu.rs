use crate::yew::context::SidebarContext;
use crate::yew::context::use_sidebar;
use crate::yew::item::MenuItem;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SubmenuProps {
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or_default]
    pub icon_html: Html,
    #[prop_or_default]
    pub children: ChildrenWithProps<MenuItem>,

    #[prop_or_default]
    pub class: &'static str,
    #[prop_or("padding: 10px; cursor: pointer; display: flex; gap: 10px; align-items: center;")]
    pub style: &'static str,
}

#[function_component(Submenu)]
pub fn submenu(props: &SubmenuProps) -> Html {
    let SidebarContext(ctx) = use_sidebar();
    let is_open = use_state(|| false);

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <>
            <div
                class={props.class}
                style={props.style}
                {onclick}
                role="button"
                aria-expanded={(*is_open).to_string()}
                aria-label={props.title}
            >
                { props.icon_html.clone() }
                if !ctx.is_collapsed {
                    <span>{ props.title }</span>
                    <span style="margin-left:auto;">{ if *is_open { "▲" } else { "▼" } }</span>
                }
            </div>
            if *is_open {
                <div style="margin-left: 1.5rem;">
                    { for props.children.iter() }
                </div>
            }
        </>
    }
}
