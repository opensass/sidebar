use crate::yew::logo::Logo;
use crate::yew::menu::Menu;
use crate::yew::{context::*, profile::Profile};
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Menu>,

    #[prop_or(true)]
    pub show_profile: bool,

    #[prop_or_default]
    pub user_name: &'static str,
    #[prop_or_default]
    pub designation: &'static str,
    #[prop_or_default]
    pub user_img: &'static str,

    #[prop_or_default]
    pub on_logout: Callback<()>,

    #[prop_or("width: 270px; background: white;")]
    pub style: &'static str,
    #[prop_or_default]
    pub class: &'static str,

    #[prop_or(
        "display: flex; justify-content: space-between; align-items: center; padding: 1rem;"
    )]
    pub header_style: &'static str,
    #[prop_or_default]
    pub header_class: &'static str,

    #[prop_or_default]
    pub logo_img_url: &'static str,
    #[prop_or_default]
    pub logo_href: &'static str,
}
#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let is_mobile = use_state(|| {
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        width < 768.0
    });

    let config = use_state(|| SidebarConfig {
        is_collapsed: *is_mobile,
    });

    let toggle_sidebar = {
        let config = config.clone();
        Callback::from(move |_| {
            config.set(SidebarConfig {
                is_collapsed: !config.is_collapsed,
            });
        })
    };
    let sidebar_style = if config.is_collapsed {
        "width: 80px; background: white;"
    } else {
        props.style
    };

    html! {
        <ContextProvider<SidebarContext> context={SidebarContext(config.clone().into())}>
            <aside class={props.class} style={format!("display: flex; flex-direction: column; height: 100vh; {}", sidebar_style)} aria-label="Sidebar Navigation">
                <div class={props.header_class} style={props.header_style}>
                    <Logo img_url={props.logo_img_url} href={props.logo_href} />
                    <button onclick={toggle_sidebar} aria-label="Toggle Sidebar">
                        { if config.is_collapsed { "▶" } else { "◀" } }
                    </button>
                </div>

                <div style="flex-grow: 1; overflow-y: auto;">
                    { for props.children.iter() }
                </div>

                if props.show_profile {
                    <Profile
                        user_name={props.user_name}
                        designation={props.designation}
                        user_img={props.user_img}
                        is_collapsed={config.is_collapsed}
                        on_logout={props.on_logout.clone()}
                    />
                }
            </aside>
        </ContextProvider<SidebarContext>>
    }
}
