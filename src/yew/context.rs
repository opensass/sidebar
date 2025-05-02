use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SidebarConfig {
    pub is_collapsed: bool,
}

#[derive(Clone, PartialEq)]
pub struct SidebarContext(pub Rc<UseStateHandle<SidebarConfig>>);

#[hook]
pub fn use_sidebar() -> SidebarContext {
    use_context::<SidebarContext>().expect("SidebarContext not found")
}
