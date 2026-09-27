use leptos::prelude::*;

use crate::{
  components::{Card, CardGrid, ExternalLink, Techs, WithIcon},
  data::{experiences::get_experience, identity::identity},
  icons,
};

#[component]
pub fn Experience() -> impl IntoView {
  let identity = identity();
  let experiences = get_experience();

  view! {
    <CardGrid>
      <Card class="md:col-span-2">
        "More details on my professional career can be read in "
        <ExternalLink href=identity.resume_url>
          <WithIcon icon=icons::DOWNLOAD_FOR_OFFLINE size="1.7em">"my resume"</WithIcon>
        </ExternalLink>
        "."
      </Card>

      {experiences
        .into_iter()
        .map(|experience| {
          view! {
            <Card class="flex flex-col">
              <div class="mb-3 rounded-md bg-white py-1 text-center">
                {experience.logo.map(|logo| view! { <img class="inline max-h-20 rounded-md align-middle" src=logo alt=experience.company /> })}
              </div>

              <h3>{experience.position}</h3>
              <p class="m-0 font-medium text-muted">
                <ExternalLink href=experience.website>{experience.company}</ExternalLink>
              </p>
              <p class="my-4 flex-1">{experience.description}</p>

              <Techs techs=experience.techs />
            </Card>
          }
        })
        .collect_view()}
    </CardGrid>
  }
}
