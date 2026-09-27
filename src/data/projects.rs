pub struct Project {
  pub name: &'static str,
  pub url: &'static str,
  pub cover: Option<&'static str>,
  pub synopsis: Vec<&'static str>,
  pub techs: Vec<&'static str>,
}

pub fn get_projects() -> Vec<Project> {
  vec![
    Project {
      name: "motiva",
      url: "https://github.com/apognu/motiva",
      cover: Some("/projects/motiva.png"),
      synopsis: vec![
        "Rust reimplementation of the matching side of OpenSanctions' Yente and nomenklatura, used to screen people and companies against sanctions lists.",
        "It serves the same search and matching API on top of an existing Yente index, with integration tests keeping its scores within a small margin of Yente's.",
      ],
      techs: vec!["Rust", "AML", "Entity recognition", "Sanctions screening"],
    },
    Project {
      name: "tuigreet",
      url: "https://github.com/apognu/tuigreet",
      cover: Some("/projects/tuigreet.png"),
      synopsis: vec![
        "Terminal-based Linux login screen for the greetd login manager.",
        "It handles authentication, session selection from desktop entries, user menus and power controls, can remember the last user and session, and is themeable and translated into several languages.",
      ],
      techs: vec!["Rust", "Linux", "greetd"],
    },
    Project {
      name: "defcon",
      url: "https://github.com/apognu/defcon",
      cover: Some("/projects/defcon.png"),
      synopsis: vec![
        "Self-hosted uptime monitoring for external services.",
        "Checks cover HTTP, TCP, UDP, ping, DNS records, TLS certificate and domain expiration, App Store and Play Store listings, dead man's switches and custom Python scripts.",
        "Runners in several locations report to a central controller, so an outage is only declared when enough sites agree, and alerts go to Slack or a webhook.",
      ],
      techs: vec!["Rust"],
    },
    Project {
      name: "pouche",
      url: "https://github.com/apognu/pouche",
      cover: Some("/projects/pouche.png"),
      synopsis: vec![
        "Webhook-to-push-notification relay.",
        "Each incoming webhook is turned into a message by a small Python adapter script, one per source, then sent through Firebase Cloud Messaging to a companion Android app, with support for banners, colors, emoji and Markdown.",
      ],
      techs: vec!["Rust", "Python"],
    },
    Project {
      name: "gocal",
      url: "https://github.com/apognu/gocal",
      cover: None,
      synopsis: vec![
        "Fast, opinionated iCalendar parser for Go.",
        "It reads events within a date window, expands recurring rules along with their exceptions and overrides, maps non-standard time zones through a callback and exposes custom X- properties.",
        "How strictly it handles errors is configurable, from rejecting a whole feed to skipping a single faulty attribute.",
      ],
      techs: vec!["Go"],
    },
    Project {
      name: "Otter",
      url: "https://github.com/apognu/otter",
      cover: Some("/projects/otter.png"),
      synopsis: vec![
        "Native Android music player for Funkwhale, the self-hosted and federated music streaming service, written in Kotlin against Funkwhale's own API rather than Subsonic.",
        "It covers library browsing, playlists, favorites, search, radios and queue management, keeps tracks available offline, and integrates with system and headset media controls.",
      ],
      techs: vec!["Android", "Kotlin", "Coroutines"],
    },
  ]
}
