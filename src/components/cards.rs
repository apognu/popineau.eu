use leptos::prelude::*;

#[component]
pub fn Card(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
  view! {
    <div class=format!("card {class}")>
      {children()}
    </div>
  }
}

#[component]
pub fn CardGrid(children: Children) -> impl IntoView {
  view! {
    <div class="grid gap-6 md:grid-cols-2 md:gap-x-8">
      {children()}
    </div>
  }
}
