mod app;
mod assets;
mod components;
mod data;

use topcoat::{
  asset::{AssetBundle, RouterBuilderAssetExt},
  router::{Router, RouterBuilderDiscoverExt},
};

#[tokio::main]
async fn main() {
  let router = Router::builder().assets(AssetBundle::load().unwrap()).discover().build();

  topcoat::start(router).await.unwrap();
}
