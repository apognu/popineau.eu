mod experience;
mod home;
mod not_found;
mod projects;

use leptos::prelude::*;
use leptos::tachys::view::iterators::StaticVec;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
  NestedRoute, StaticSegment,
  components::{RouteChildren, Router, Routes, RoutesProps},
  hooks::use_location,
};

use self::{experience::Experience, home::Home, not_found::NotFound, projects::Projects};
use crate::data::identity::identity;

type PageSpec<'s> = (&'s str, &'s str, fn() -> AnyView);

const PAGES: [PageSpec; 3] = [
  ("/", "who am i?", || view! { <Home /> }.into_any()),
  ("/experience", "experience", || view! { <Experience /> }.into_any()),
  ("/projects", "projects", || view! { <Projects /> }.into_any()),
];

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />

        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <MetaTags />
      </head>

      <body>
        <App />
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();

  let identity = identity();
  let description = format!("{}, {} - {} at {}", identity.fullname(), identity.title, identity.employer.title, identity.employer.name);

  view! {
    <Stylesheet id="leptos" href="/pkg/popineau_eu.css" />

    <Meta name="description" content=description.clone() />
    <Meta property="og:title" content=identity.fullname() />
    <Meta property="og:description" content=description />
    <Meta property="og:image" content=format!("{}{}", identity.website, identity.picture) />
    <Meta property="og:url" content=identity.website />
    <Meta name="twitter:card" content="summary_large_image" />

    <Router>
      <div class="mx-auto max-w-[1200px]">
        <Header />
        <Nav />

        <PageRoutes />
      </div>
    </Router>
  }
}

#[component]
fn PageRoutes() -> impl IntoView {
  Routes(
    RoutesProps::builder()
      .fallback(NotFound)
      .children(RouteChildren::to_children(|| {
        StaticVec::from(PAGES.map(|(url, _, view)| NestedRoute::new(StaticSegment(url.trim_start_matches('/')), view)).to_vec())
      }))
      .build(),
  )
}

#[component]
fn Header() -> impl IntoView {
  let pathname = use_location().pathname;
  let title = move || {
    let current_url = pathname.get();

    PAGES.iter().find(|(url, _, _)| *url == current_url).map(|(_, title, _)| *title).unwrap_or("page not found")
  };

  let identity = identity();
  let fullname = identity.fullname();

  view! {
    <Title text=move || format!("{} - {}", fullname, title()) />

    <header class="flex gap-4">
      <div class="mb-1 flex-1 lowercase"><h1>"$ " {identity.firstname} " " <span>{identity.lastname}</span></h1></div>
      <div class="shrink-0 text-right"><h2>{title}</h2></div>
    </header>
  }
}

#[component]
fn Nav() -> impl IntoView {
  let pathname = use_location().pathname;

  view! {
    <nav class="mb-12">
      <ul class="grid grid-cols-3 md:block">
        {PAGES
          .into_iter()
          .map(|(url, title, _)| {
            let is_current = move || pathname.get() == url;

            view! {
              <li class="mx-4 pt-1 text-center md:mx-0 md:mr-6 md:inline-block md:text-left">
                <a
                  href=url
                  class="block py-4 outline-none hover:border-b-2 hover:border-accent hover:no-underline focus:border-b-2 focus:border-accent md:inline md:py-2"
                  class=(["border-b-2", "border-accent", "text-accent"], is_current)
                  class:text-fg=move || !is_current()
                >
                  {title}
                </a>
              </li>
            }
          })
          .collect_view()}
      </ul>
    </nav>
  }
}
