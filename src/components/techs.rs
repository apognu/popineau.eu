use topcoat::{
  Result,
  view::{View, component, view},
};

#[component]
pub async fn tech_list(techs: Vec<&'static str>) -> Result<impl View> {
  Ok(view! {
    <div>
      for tech in techs {
        <span class="pill">(tech)</span>
      }
    </div>
  })
}
