#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
  use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
  };
  use leptos::prelude::*;
  use leptos_axum::{LeptosRoutes, generate_route_list};
  use popineau_eu::app::{App, shell};
  use tracing_subscriber::EnvFilter;

  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
    .init();

  let conf = get_configuration(None).unwrap();
  let options = conf.leptos_options;
  let addr = options.site_addr;
  let routes = generate_route_list(App);

  let app = Router::new()
    .route("/-/readyz", get(|| async {}))
    .leptos_routes(&options, routes, {
      let options = options.clone();
      move || shell(options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .with_state(options)
    .layer(middleware::from_fn(access_log));

  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

  tracing::info!("listening on http://{addr}");

  axum::serve(listener, app.into_make_service()).await.unwrap();

  async fn access_log(request: Request, next: Next) -> Response {
    if request.uri().path().starts_with("/-/") {
      return next.run(request).await;
    }

    let method = request.method().clone();
    let path = request.uri().path().to_owned();
    let start = std::time::Instant::now();
    let response = next.run(request).await;

    tracing::info!(status = response.status().as_u16(), latency = ?start.elapsed(), "{method} {path}");

    response
  }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
