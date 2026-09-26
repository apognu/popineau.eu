use topcoat::{
  Result,
  router::page,
  view::{View, view},
};
use topcoat_css::css;

use crate::{components::external_link, data::experiences::get_experience};

#[page("/experience")]
async fn experience() -> Result<impl View> {
  let style = css! {
    .experiences {
      display: grid;
      grid-template-columns: 0.5fr 0.5fr;
      grid-column-gap: 32px;

      @media (width <= 768px) {
        display: block;
      }

      .experience {
        display: flex;
        flex-direction: column;
        margin-bottom: 32px;
        padding: 16px;
        background: var(--surface);
        border-radius: 6px;

        .logo {
          background: white;
          margin-bottom: 12px;
          border-radius: 6px;
          text-align: center;
          padding: 4px 0;

          img {
            display: inline;
            max-height: 80px;
            border-radius: 6px;
            vertical-align: middle;

            @media (width <= 768px) {
              width: auto;
            }
          }
        }

        .company {
          margin: 0;
          font-weight: 500;
          color: var(--dark-gray);
        }

        .description {
          margin: 16px 0;
          flex: 1;
        }
      }
    }
  };

  let experiences = get_experience();

  Ok(view! {
    <div class=(style.experiences)>
      for experience in experiences {
        <div class=(style.experience)>
          <div class=(style.logo)>
            if experience.logo.is_some() {
              <img src=(experience.logo) alt=(experience.company)>
            }
          </div>

          <h3>(experience.position)</h3>
          <p class=(style.company)>
            external_link(url: experience.website, (experience.company))
          </p>
          <p class=(style.description)>(experience.description)</p>

          <div class="techs">
            for tech in experience.techs {
              <span class="pill">(tech)</span>
            }
          </div>
        </div>
      }
    </div>
  })
}
