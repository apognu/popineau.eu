use topcoat::{
  Result,
  router::page,
  view::{View, view},
};
use topcoat_css::css;

use crate::{
  assets::{PICTURE, material_symbols_light as icons},
  components::{external_link, icon, social_network},
  data::identity::identity,
};

#[page("/")]
async fn home() -> Result<impl View> {
  let style = css! {
    :root {
      --avatar-size: 250px;
      --avatar-margin: 16px;
    }

    .identity {
      img {
        float: left;
        width: var(--avatar-size);
        margin-right: var(--avatar-margin);
        margin-bottom: 8px;
        border-radius: 6px;

        @media (width <= 768px) {
          display: block;
          margin-left: auto;
          margin-right: auto;
          margin-bottom: 16px;
          float: none;
        }
      }

      .data {
        background: var(--surface);
        margin-left: calc(var(--avatar-size) + var(--avatar-margin));
        padding: 16px;
        border-radius: 6px;

        @media (width <= 768px) {
          margin-left: 0;
        }
      }

      .card {
        display: grid;
        grid-template-columns: 1fr auto;
        gap: 8px;

        h3 {
          font-size: 1.5rem;
          font-weight: 500;

          span {
            font-size: 0.8rem;
            font-weight: normal;
            margin-left: 10px;
            color: var(--dark-gray);
          }
        }

        @media (width <= 768px) {
          grid-template-columns: 1fr;
        }
      }

      h4 {
        font-size: 1.1rem;
        color: var(--dark-gray);
      }

      .position {
        margin-top: 24px;
        margin-bottom: 2px;

        .title {
          font-weight: 500;
        }

        .company {
          color: var(--dark-gray);
        }

        .more-experiences {
          font-size: 0.9rem;
          color: var(--dark-gray);
          text-decoration: underline;

          &:hover {
            text-decoration: none;
          }
        }
      }

      .location {
        margin-top: 8px;
        margin-bottom: 24px;
      }

      .gpg {
        margin-bottom: 32px;

        p {
          margin: 8px 0 16px 0;

          span {
            color: white;

            @media (width <= 768px) {
              display: block;
            }
          }
        }
      }
    }
  };

  let identity = identity();

  Ok(view! {
    <div class=(style.identity)>
      <img src=(PICTURE) :alt=(identity.fullname()) />

      <div class=(style.data)>
        <div class=(style.card)>
          <div>
            <h3>
              (identity.fullname())
              <span>(identity.age) " years old"</span>
            </h3>

            <h4>(identity.title)</h4>
          </div>

          <p>
            <a class="btn" href=(identity.resume_url)>
              icon(data: icons::DOWNLOAD, class: "pr-3")
              "Download resume"
            </a>
          </p>
        </div>

        <p class=(style.position)>
          icon(data: icons::BUSINESS_CENTER_ROUNDED, class: "pr-3")
          <span class=(style.title)>(identity.employer.title)</span>
          " at "
          external_link(url: identity.employer.website, (identity.employer.name))
          " - "
          <a href="/experience" class=(style.more_experiences)>"see more"</a>
        </p>

        <p class=(style.location)>
          icon(data: icons::LOCATION_ON, class: "pr-3")
          <span class=(style.title)>(identity.location)</span>
        </p>

        <div class=(style.gpg)>
          <p>
            "GPG fingerprint:"
            <span>
                (identity.gpg.fingerprint)
                <a href=(identity.gpg.url)>
                  icon(data: icons::DOWNLOAD_FOR_OFFLINE, class: "pl-3")
                </a>
            </span>
          </p>
        </div>

        for network in identity.socials {
          social_network(network: network)
        }
      </div>
    </div>
  })
}
