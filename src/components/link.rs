use topcoat::{
  Result,
  asset::Asset,
  view::{Child, View, component, view},
};

pub enum LinkTarget {
  Asset(Asset),
  External(&'static str),
  Dynamic(String),
}

impl From<Asset> for LinkTarget {
  fn from(asset: Asset) -> Self {
    LinkTarget::Asset(asset)
  }
}

impl From<&'static str> for LinkTarget {
  fn from(url: &'static str) -> Self {
    LinkTarget::External(url)
  }
}

impl From<String> for LinkTarget {
  fn from(url: String) -> Self {
    LinkTarget::Dynamic(url)
  }
}

#[component]
pub async fn external_link(#[into] url: LinkTarget, child: Child<'_>) -> Result<impl View> {
  Ok(view! {
    <a
      match url {
        LinkTarget::Asset(asset) => href=(asset),
        LinkTarget::External(url) => href=(url),
        LinkTarget::Dynamic(url) => href=(url),
      }
      target="_blank"
      rel="noopener noreferrer">

      (child)
    </a>
  })
}
