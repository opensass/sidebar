use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LogoProps {
    #[prop_or("/")]
    pub href: &'static str,
    #[prop_or_default]
    pub img_url: &'static str,

    #[prop_or("padding: 1rem;")]
    pub style: &'static str,
    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    html! {
        <a href={props.href} class={props.class} style={props.style} aria-label="Home">
            if props.img_url.is_empty() {
                <strong>{ "LOGO" }</strong>
            } else {
                <img src={props.img_url} alt="Logo" style="max-height: 40px;" />
            }
        </a>
    }
}
