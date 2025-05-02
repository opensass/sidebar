use crate::yew::context::SidebarContext;
use crate::yew::context::use_sidebar;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub sub_heading: &'static str,
    #[prop_or("padding: 1rem;")]
    pub style: &'static str,
    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProps) -> Html {
    let SidebarContext(ctx) = use_sidebar();

    html! {
        <div
            class={props.class}
            style={props.style}
            aria-label="Sidebar Menu Section"
        >
            if !ctx.is_collapsed {
                <h4 style="font-size: 12px; font-weight: bold;">{ props.sub_heading }</h4>
            }
            { for props.children.iter() }
        </div>
    }
}
