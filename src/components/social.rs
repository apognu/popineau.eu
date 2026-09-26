use topcoat::{
  Result,
  view::{View, component, view},
};

use crate::{
  assets::fa7_brands as brands,
  components::{external_link, with_icon},
  data::identity::Social,
};

#[component]
pub async fn social_network(network: Social) -> Result<impl View> {
  let data = match network.network {
    "GitHub" => Some(brands::GITHUB),
    "LinkedIn" => Some(brands::LINKEDIN),
    _ => None,
  };

  Ok(view! {
    <p class="mb-2">
      with_icon(
        data: data,
        external_link(
          url: network.url,
          (network.handle) " on " (network.network)
        )
      )
    </p>
  })
}
