use topcoat::asset::Asset;

use crate::assets::*;

pub struct Project {
  pub name: &'static str,
  pub url: &'static str,
  pub cover: Option<Asset>,
  pub synopsis: &'static str,
  pub techs: Vec<&'static str>,
}

pub fn get_projects() -> Vec<Project> {
  vec![
    Project {
      name: "motiva",
      url: "https://github.com/apognu/motiva",
      cover: Some(MOTIVA),
      synopsis: "Reimplementation of Open Sanctions' sanctions screening algorithms and API.",
      techs: vec!["Rust", "AML", "Entity recognition", "Sanctions screening"],
    },
    Project {
      name: "tuigreet",
      url: "https://github.com/apognu/tuigreet",
      cover: Some(TUIGREET),
      synopsis: "Graphical console greeter for the greetd login manager.",
      techs: vec!["Rust", "greetd"],
    },
    Project {
      name: "defcon",
      url: "https://github.com/apognu/defcon",
      cover: Some(DEFCON),
      synopsis: "Uptime-like external monitoring system to alert on outages for HTTP and TCP services, app listing on mobile stores and TLS certificates issues.",
      techs: vec!["Rust"],
    },
    Project {
      name: "pouche",
      url: "https://github.com/apognu/pouche",
      cover: Some(POUCHE),
      synopsis: "Extensible webhook server to dispatch mobile push notifications",
      techs: vec!["Rust", "Python"],
    },
    Project {
      name: "gocal",
      url: "https://github.com/apognu/gocal",
      cover: None,
      synopsis: "Efficient library to parse iCal documents in Go programs.",
      techs: vec!["Go"],
    },
    Project {
      name: "Otter",
      url: "https://github.com/apognu/otter",
      cover: Some(OTTER),
      synopsis: "Native Android music player for remote Open Source streaming service Funkwhale.",
      techs: vec!["Android", "Kotlin", "Coroutines"],
    },
  ]
}
