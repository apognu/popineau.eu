use topcoat::{
  Result,
  icon::IconData,
  view::{Attributes, Length, View, component, view},
};

#[component]
pub async fn icon(
  data: IconData,
  #[into]
  #[default(Length::em(1.5))]
  size: Length,
  #[default]
  #[into]
  label: String,
  #[default]
  #[into]
  class: String,
  #[default] attrs: Attributes,
) -> Result<impl View> {
  Ok(view! {
    <span class=(class)>
      <svg
          viewBox=(data.view_box())
          width=(size)
          height=(size)
          aria-hidden=(label.is_empty().then_some("true"))
          role=((!label.is_empty()).then_some("img"))
          aria-label=((!label.is_empty()).then_some(label))
          (attrs)
      >
          (data.into_body())
      </svg>
    </span>
  })
}
