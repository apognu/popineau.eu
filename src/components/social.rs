use topcoat::{
  Result,
  view::{View, component, view},
};

use crate::{
  assets::fa7_brands as brands,
  components::{external_link, icon},
  data::identity::Social,
};

#[component]
pub async fn social_network(network: Social) -> Result<impl View> {
  Ok(view! {
    <p class="mb-2">
      match network.network {
        "GitHub" => icon(data: brands::GITHUB, class: "pr-3"),
        "LinkedIn" => icon(data: brands::LINKEDIN, class: "pr-3"),
        _ => {}
      }

      external_link(
        url: network.url,
        (network.handle) " on " (network.network)
      )
    </p>
  })
}
