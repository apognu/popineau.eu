use jiff::{SpanRound, Unit, Zoned, civil::Date};

pub struct Identity {
  pub website: &'static str,
  pub picture: &'static str,
  pub resume_url: &'static str,
  pub firstname: &'static str,
  pub lastname: &'static str,
  pub age: i16,
  pub location: &'static str,
  pub title: &'static str,
  pub gpg: Gpg,
  pub employer: Employer,
  pub github_url: &'static str,
  pub socials: Vec<Social>,
}

impl Identity {
  pub fn fullname(&self) -> String {
    format!("{} {}", self.firstname, self.lastname)
  }
}

pub struct Gpg {
  pub fingerprint: &'static str,
  pub url: &'static str,
}

pub struct Employer {
  pub name: &'static str,
  pub title: &'static str,
  pub website: &'static str,
}

pub struct Social {
  pub network: &'static str,
  pub handle: &'static str,
  pub url: &'static str,
}

pub fn identity() -> Identity {
  let birth = Date::constant(1989, 9, 10);
  let age = Zoned::now()
    .date()
    .since(birth)
    .unwrap_or_default()
    .round(SpanRound::new().largest(Unit::Year).relative(Zoned::now().date()))
    .unwrap_or_default()
    .get_years();

  Identity {
    website: "https://popineau.eu",
    picture: "/picture.jpg",
    resume_url: "https://github.com/apognu/resume/releases/download/latest/Antoine.POPINEAU.-.Resume.pdf",
    firstname: "Antoine",
    lastname: "POPINEAU",
    age,
    location: "Paris, France",
    title: "Infrastructure architect, DevOps & systems developer",
    gpg: Gpg {
      fingerprint: "1F90 EA90 4869 0EA5 E053 D80F 121C 64AF D045 C39C",
      url: "https://keys.openpgp.org/vks/v1/by-fingerprint/9A9BBBB458D526BECF16DD2E8A3DFF41E10C6E27",
    },
    employer: Employer {
      name: "Marble",
      website: "https://www.checkmarble.com",
      title: "Senior Backend Developer",
    },
    github_url: "https://github.com/apognu",
    socials: vec![
      Social {
        network: "GitHub",
        handle: "apognu",
        url: "https://github.com/apognu",
      },
      Social {
        network: "LinkedIn",
        handle: "antoinepopineau",
        url: "https://linkedin.com/in/antoinepopineau",
      },
    ],
  }
}
