#![doc(
    html_logo_url = "https://raw.githubusercontent.com/opensass/sidebar/refs/heads/main/assets/logo.webp",
    html_favicon_url = "https://github.com/opensass/sidebar/blob/main/assets/favicon.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "yew")]
pub mod yew;

#[cfg(feature = "dio")]
pub mod dioxus;

#[cfg(feature = "lep")]
pub mod leptos;
