use topcoat::{
  Result,
  router::page,
  view::{View, view},
};

use crate::{
  assets::fa7_brands as brands,
  components::{card, card_grid, external_link, icon, tech_list},
  data::{identity::identity, projects::get_projects},
};

#[page("/projects")]
async fn projects() -> Result<impl View> {
  let identity = identity();
  let projects = get_projects();

  Ok(view! {
    card_grid(
      card(
        class: "md:col-span-2",
        "See more projects and contributions on "
        external_link(
          url: identity.github_url,
          icon(data: brands::GITHUB) " GitHub."
        )
      )

      #[key(project.name)]
      for project in projects {
        card(
          class: "flex flex-col",
          if let Some(cover) = project.cover {
            external_link(
              url: cover,
              <img class="mb-3 w-full rounded-md" src=(cover) alt=(project.name)>
            )
          }

          <div class="flex flex-1 flex-col">
            <h3>
              external_link(url: project.url, (project.name))
            </h3>

            <p class="my-4 flex-1">(project.synopsis)</p>

            tech_list(techs: project.techs)
          </div>
        )
      }
    )
  })
}
