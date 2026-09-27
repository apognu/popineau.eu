use leptos::prelude::*;

use crate::{
  components::{Card, CardGrid, ExternalLink, Techs, WithIcon},
  data::{identity::identity, projects::get_projects},
  icons,
};

#[component]
pub fn Projects() -> impl IntoView {
  let identity = identity();
  let projects = get_projects();

  view! {
    <CardGrid>
      <Card class="md:col-span-2">
        "See more projects and contributions on "
        <ExternalLink href=identity.github_url>
          <WithIcon icon=icons::GITHUB>"GitHub"</WithIcon>
        </ExternalLink>
        "."
      </Card>

      {projects
        .into_iter()
        .map(|project| {
          view! {
            <Card class="flex flex-col">
              {project
                .cover
                .map(|cover| {
                  view! {
                    <ExternalLink href=cover>
                      <img class="mb-3 w-full rounded-md" src=cover alt=project.name />
                    </ExternalLink>
                  }
                })}

              <div class="flex flex-1 flex-col">
                <h3>
                  <ExternalLink href=project.url>{project.name}</ExternalLink>
                  {project.url.strip_prefix("https://github.com/").map(|repo| view! { <Stars repo=repo /> })}
                </h3>

                <div class="my-4 flex-1 space-y-2">
                  {project.synopsis.into_iter().map(|paragraph| view! { <p>{paragraph}</p> }).collect_view()}
                </div>

                <Techs techs=project.techs />
              </div>
            </Card>
          }
        })
        .collect_view()}
    </CardGrid>
  }
}

#[component]
fn Stars(repo: &'static str) -> impl IntoView {
  let stars = LocalResource::new(move || get_stars(repo.to_string()));

  view! {
    {move || {
      stars.get().and_then(Result::ok).map(|count| view! { <span class="ml-2 text-sm text-muted"><span class="text-star">"★"</span> " " {count}</span> })
    }}
  }
}

#[server]
async fn get_stars(repo: String) -> Result<u64, ServerFnError> {
  fetch_stars(repo).await
}

#[cfg(feature = "ssr")]
#[cached::cached(ttl_secs = 3600, sync_writes = "by_key")]
async fn fetch_stars(repo: String) -> Result<u64, ServerFnError> {
  #[derive(serde::Deserialize)]
  struct Repository {
    stargazers_count: u64,
  }

  let repository = tokio::task::spawn_blocking(move || {
    ureq::get(format!("https://api.github.com/repos/{repo}"))
      .header("Accept", "application/vnd.github+json")
      .header("User-Agent", "popineau.eu")
      .call()?
      .body_mut()
      .read_json::<Repository>()
  })
  .await
  .map_err(ServerFnError::new)?
  .map_err(ServerFnError::new)?;

  Ok(repository.stargazers_count)
}
