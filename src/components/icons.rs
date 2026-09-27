use leptos::prelude::*;

#[component]
pub fn Icon(icon: &'static str, #[prop(default = "1.5em")] size: &'static str, #[prop(optional, into)] class: String) -> impl IntoView {
  view! { <span class=format!("icon {class}") style=format!("--icon-size: {size}") aria-hidden="true" inner_html=icon></span> }
}

#[component]
pub fn WithIcon(#[prop(into)] icon: Option<&'static str>, #[prop(default = "1.5em")] size: &'static str, children: Children) -> impl IntoView {
  view! {
    {icon.map(|icon| view! { <Icon icon size class="pr-2" /> })}
    {children()}
  }
}
