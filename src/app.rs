mod experience;
mod home;
mod projects;

use topcoat::{
  Result,
  context::Cx,
  router::{Slot, layout, request::uri},
  view::{View, component, view},
};

use crate::assets::SPACE_MONO;

const PAGES: [(&str, &str); 3] = [("/", "who am i?"), ("/experience", "experience"), ("/projects", "projects")];

#[layout("/")]
async fn root_layout(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
  let title = || {
    let current_url = uri(cx).path();

    PAGES.iter().find(|(url, _)| *url == current_url).map(|(_, title)| *title).unwrap_or("page not found")
  };

  Ok(view! {
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">

        <meta name="description" content="Antoine POPINEAU, Infrastructure architect &amp; DevOps - CTO at AppScho">
        <meta property="og:title" content="Antoine POPINEAU">
        <meta property="og:description" content="Antoine POPINEAU, Infrastructure architect &amp; DevOps - CTO at AppScho">
        <meta property="og:image" content="https://popineau.eu/assets/picture.jpg">
        <meta property="og:url" content="https://popineau.eu">
        <meta name="twitter:card" content="summary_large_image">

        <link rel="stylesheet" href=(topcoat::tailwind::stylesheet!())>
        topcoat::font::link(font: SPACE_MONO)

        <title>"antoine popineau - " (title())</title>
      </head>

      <body>
        <div class="mx-auto max-w-[1200px]">
          <header class="flex gap-4">
            <div class="mb-1 flex-1"><h1>"$ antoine " <span>"popineau"</span></h1></div>
            <div class="shrink-0 text-right"><h2>(title())</h2></div>
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
  let class = "block py-4 outline-none hover:border-b-2 hover:border-accent hover:no-underline focus:border-b-2 focus:border-accent md:inline md:py-2";

  Ok(view! {
    <nav class="mb-12">
      <ul class="grid grid-cols-3 md:block">
        for (url, title) in PAGES {
          <li class="mx-4 pt-1 text-center md:mx-0 md:mr-6 md:inline-block md:text-left">
            <a class=(format!("{class} {}", if is_current(url) { "border-b-2 border-accent text-accent" } else { "text-fg" })) href=(url)>(title)</a>
          </li>
        }
      </ul>
    </nav>
  })
}
