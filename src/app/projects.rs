use topcoat::{
  Result,
  router::page,
  view::{View, view},
};
use topcoat_css::css;

use crate::{
  assets::fa7_brands as brands,
  components::{external_link, icon},
  data::{identity::identity, projects::get_projects},
};

#[page("/projects")]
async fn projects() -> Result<impl View> {
  let style = css! {
    .projects {
      display: grid;
      grid-template-columns: 0.5fr 0.5fr;
      grid-column-gap: 32px;
      grid-row-gap: 24px;

      @media (width <= 768px) {
        display: block;
      }
    }

    .more {
      grid-column: 1 / 3;
      padding: 16px;
      background: var(--surface);
      border-radius: 6px;
    }

    .project {
      display: flex;
      flex-direction: column;
      padding: 16px;
      background: var(--surface);
      border-radius: 6px;

      img {
        width: 150px;
        border-radius: 6px;
        margin-bottom: 12px;
        width: 100%;
      }

      > div {
        flex: 1;
        display: flex;
        flex-direction: column;

        .synopsis {
          margin: 0;
          margin: 16px 0;
          flex: 1;
        }
      }
    }
  };

  let identity = identity();
  let projects = get_projects();

  Ok(view! {
    <div>
      <div class=(style.projects)>
        <p class=(style.more)>
          "See more projects and contributions on "
          external_link(
            url: identity.github_url,
            icon(data: brands::GITHUB) " GitHub."
          )
        </p>


        for project in projects {
          <div class=(style.project)>
            if let Some(cover) = project.cover {
              external_link(
                url: cover,
                <img src=(cover) alt=(project.name)>
              )
            }

            <div>
              <h3>
                external_link(url: project.url, (project.name))
              </h3>

              <p class=(style.synopsis)>(project.synopsis)</p>

              <div class="techs">
                for tech in project.techs {
                  <span class="pill">(tech)</span>
                }
              </div>
            </div>
          </div>
        }
      </div>
    </div>
  })
}
