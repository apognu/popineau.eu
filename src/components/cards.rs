use topcoat::{
  Result,
  view::{Child, View, component, view},
};

#[component]
pub async fn card(
  #[default]
  #[into]
  class: String,
  child: Child<'_>,
) -> Result<impl View> {
  Ok(view! {
    <div class=(format!("card {class}"))>
      (child)
    </div>
  })
}

#[component]
pub async fn card_grid(child: Child<'_>) -> Result<impl View> {
  Ok(view! {
    <div class="grid gap-6 md:grid-cols-2 md:gap-x-8">
      (child)
    </div>
  })
}
