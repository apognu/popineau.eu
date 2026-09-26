use topcoat::{
  Result,
  router::page,
  view::{View, view},
};

use crate::{
  assets::{PICTURE, material_symbols_light as icons},
  components::{card, external_link, icon, social_network, with_icon},
  data::identity::identity,
};

#[page("/")]
async fn home() -> Result<impl View> {
  let identity = identity();

  Ok(view! {
    <div>
      <img class="mx-auto mb-4 w-avatar rounded-md md:float-left md:mr-4 md:mb-2" src=(PICTURE) :alt=(identity.fullname()) />

      card(
        class: "md:ml-[calc(var(--spacing-avatar)+1rem)]",
        <div class="grid grid-cols-1 gap-2 md:grid-cols-[1fr_auto]">
          <div>
            <h3 class="text-2xl font-medium">
              (identity.fullname())
              <span class="ml-2.5 text-xs font-normal text-muted">(identity.age) " years old"</span>
            </h3>

            <h4 class="text-lg text-muted">(identity.title)</h4>
          </div>

          <p class="my-3 md:my-0">
            <a class="btn" href=(identity.resume_url)>
              with_icon(data: icons::DOWNLOAD, "Download resume")
            </a>
          </p>
        </div>

        <p class="mt-6 mb-0.5">
          with_icon(
            data: icons::BUSINESS_CENTER_ROUNDED,
            <span class="font-medium">(identity.employer.title)</span>
            " at "
            external_link(url: identity.employer.website, (identity.employer.name))
            " - "
            <a href="/experience" class="text-sm text-muted underline hover:no-underline">"see more"</a>
          )
        </p>

        <p class="mt-2 mb-6">
          with_icon(data: icons::LOCATION_ON, <span class="font-medium">(identity.location)</span>)
        </p>

        <div class="mb-8">
          <p class="mt-2 mb-4 hidden md:block">
            "GPG fingerprint:"
            <span class="ml-2 text-white">(identity.gpg.fingerprint)</span>
            <a class="ml-3" href=(identity.gpg.url)>
              icon(data: icons::DOWNLOAD_FOR_OFFLINE)
            </a>
          </p>

          <p class="mt-2 mb-4 md:hidden">
            "GPG public key:"
            <a class="btn ml-2" href=(identity.gpg.url)>
              with_icon(data: icons::DOWNLOAD_FOR_OFFLINE, "Download")
            </a>
          </p>
        </div>

        #[key(network.network)]
        for network in identity.socials {
          social_network(network: network)
        }
      )
    </div>
  })
}
