use topcoat::{
  Result,
  router::page,
  view::{View, view},
};

use crate::{
  components::{card, card_grid, external_link, tech_list},
  data::experiences::get_experience,
};

#[page("/experience")]
async fn experience() -> Result<impl View> {
  let experiences = get_experience();

  Ok(view! {
    card_grid(
      #[key(experience.company)]
      for experience in experiences {
        card(
          class: "flex flex-col",
          <div class="mb-3 rounded-md bg-white py-1 text-center">
            if experience.logo.is_some() {
              <img class="inline max-h-20 rounded-md align-middle" src=(experience.logo) alt=(experience.company)>
            }
          </div>

          <h3>(experience.position)</h3>
          <p class="m-0 font-medium text-muted">
            external_link(url: experience.website, (experience.company))
          </p>
          <p class="my-4 flex-1">(experience.description)</p>

          tech_list(techs: experience.techs)
        )
      }
    )
  })
}
