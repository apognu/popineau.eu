mod experience;
mod home;
mod projects;

use topcoat::{
  Result,
  context::Cx,
  router::{Slot, layout, request::uri},
  view::{View, component, view},
};
use topcoat_css::css;

use crate::assets::SPACE_MONO;

const PAGES: [(&str, &str); 3] = [("/", "who am i?"), ("/experience", "experience"), ("/projects", "projects")];

#[layout("/")]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
  let style = css! {
    .container {
      max-width: 1200px;
      margin: auto;
    }

    header {
      display: flex;

      > div {
        flex: 1;

        &:nth-child(1) {
          margin-bottom: 4px;
        }

        &:nth-child(2) {
          text-align: right;
        }
      }
    }
  };

  let title = || {
    let current_url = uri(cx).path();

    PAGES.iter().find(|(url, _)| *url == current_url).map(|(_, title)| *title).unwrap_or("page not found")
  };

  Ok(view! {
    <!DOCTYPE html>
    <html>
      <head>
        <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
        <link rel="stylesheet" href=(topcoat_css::stylesheet!())>
        topcoat::font::link(font: SPACE_MONO)

        <title>"antoine popineau - " (title())</title>
      </head>

      <body>
        <div class=(style.container)>
          <header>
            <div><h1>"$ antoine " <span>"popineau"</span></h1></div>
            <div><h2>(title())</h2></div>
          </header>

          nav()

          (slot)
        </div>
      </body>
    </html>
  })
}

#[component]
async fn nav(cx: &Cx) -> Result<impl View> {
  let is_current = |url: &str| uri(cx).path() == url;

  let style = css! {
    nav {
      margin-bottom: 48px;

      ul {
        list-style: none;
        margin: 0;
        padding: 0;

        li {
          display: inline-block;
          margin-right: 24px;
          padding-top: 4px;

          a {
            color: var(--normal);
            outline: 0;
            padding: 8px 0;

            &.active {
              border-bottom: 2px solid var(--accent);
              color: var(--accent);
            }

            &:hover,
            &:focus {
              text-decoration: none;
              border-bottom: 2px solid var(--accent);
            }
          }
        }

        @media (width <= 768px) {
          display: grid;
          grid-template-columns: 1fr 1fr 1fr;

          li {
            text-align: center;
            margin: 0 16px;

            a {
              display: block;
              padding: 16px 0;
            }
          }
        }
      }
    }
  };

  Ok(view! {
    <nav>
      <ul>
        for (url, title) in PAGES {
          <li><a if is_current(url) { class=(style.active) } href=(url)>(title)</a></li>
        }
      </ul>
    </nav>
  })
}
