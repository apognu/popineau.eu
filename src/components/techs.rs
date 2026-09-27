use leptos::prelude::*;

#[component]
pub fn Techs(techs: Vec<&'static str>) -> impl IntoView {
  view! {
    <div>
      {techs.into_iter().map(|tech| view! { <span class="pill">{tech}</span> }).collect_view()}
    </div>
  }
}
