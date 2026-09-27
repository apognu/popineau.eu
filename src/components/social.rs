use leptos::prelude::*;

use crate::{
  components::{ExternalLink, WithIcon},
  data::identity::Social,
  icons,
};

#[component]
pub fn SocialNetwork(network: Social) -> impl IntoView {
  let icon = match network.network {
    "GitHub" => Some(icons::GITHUB),
    "LinkedIn" => Some(icons::LINKEDIN),
    _ => None,
  };

  view! {
    <p class="mb-2">
      <WithIcon icon=icon>
        <ExternalLink href=network.url>
          {network.handle} " on " {network.network}
        </ExternalLink>
      </WithIcon>
    </p>
  }
}
