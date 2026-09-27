use leptos::prelude::*;

use crate::{
  components::{Card, ExternalLink, Icon, SocialNetwork, WithIcon},
  data::identity::identity,
  icons,
};

#[component]
pub fn Home() -> impl IntoView {
  let identity = identity();

  view! {
    <div>
      <img class="mx-auto mb-4 w-avatar rounded-md md:float-left md:mr-4 md:mb-2" src=identity.picture alt=identity.fullname() />

      <Card class="md:ml-[calc(var(--spacing-avatar)+1rem)]">
        <div class="grid grid-cols-1 gap-2 md:grid-cols-[1fr_auto]">
          <div>
            <h3 class="text-2xl font-medium">
              {identity.fullname()}
              <span class="ml-2.5 text-xs font-normal text-muted">{identity.age} " years old"</span>
            </h3>

            <h4 class="text-lg text-muted">{identity.title}</h4>
          </div>

          <p class="my-3 md:my-0">
            <a class="btn" href=identity.resume_url>
              <WithIcon icon=icons::DOWNLOAD>"Download resume"</WithIcon>
            </a>
          </p>
        </div>

        <p class="mt-6 mb-0.5">
          <WithIcon icon=icons::BUSINESS_CENTER_ROUNDED>
            <span class="font-medium">{identity.employer.title}</span>
            " at "
            <ExternalLink href=identity.employer.website>{identity.employer.name}</ExternalLink>
            " - "
            <a href="/experience" class="text-sm text-muted underline hover:no-underline">"see more"</a>
          </WithIcon>
        </p>

        <p class="mt-2 mb-6">
          <WithIcon icon=icons::LOCATION_ON>
            <span class="font-medium">{identity.location}</span>
          </WithIcon>
        </p>

        <div class="mb-8">
          <p class="mt-2 mb-4 hidden md:block">
            "GPG fingerprint:"
            <span class="ml-2 text-white">{identity.gpg.fingerprint}</span>
            <a class="ml-3" href=identity.gpg.url>
              <Icon icon=icons::DOWNLOAD_FOR_OFFLINE size="1.7em" />
            </a>
          </p>

          <p class="mt-2 mb-4 md:hidden">
            "GPG public key:"
            <a class="btn ml-2" href=identity.gpg.url>
              <WithIcon icon=icons::DOWNLOAD_FOR_OFFLINE size="1.7em">"Download"</WithIcon>
            </a>
          </p>
        </div>

        {identity.socials.into_iter().map(|network| view! { <SocialNetwork network=network /> }).collect_view()}
      </Card>
    </div>
  }
}
