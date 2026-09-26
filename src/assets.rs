use topcoat::{
  asset::{Asset, asset},
  font::{Font, fontsource::fontsource_font},
  icon::iconify,
};

pub const SPACE_MONO: Font = fontsource_font!(SPACE_MONO, host: Asset);

iconify::include!(pub "material-symbols-light");
iconify::include!(pub "fa7-brands");

pub const PICTURE: Asset = asset!("../assets/picture.jpg");

pub const APPSCHO: Asset = asset!("../assets/experiences/appscho.png");
pub const HELLO_TOMORROW: Asset = asset!("../assets/experiences/hellotomorrow.png");
pub const INA: Asset = asset!("../assets/experiences/ina.png");
pub const MARBLE: Asset = asset!("../assets/experiences/marble.png");
pub const READY_EDUCATION: Asset = asset!("../assets/experiences/readyeducation.png");
pub const SFR_BUSINESS: Asset = asset!("../assets/experiences/sfrbusiness.png");
pub const SMILE: Asset = asset!("../assets/experiences/smile.png");

pub const DEFCON: Asset = asset!("../assets/projects/defcon.png");
pub const MOTIVA: Asset = asset!("../assets/projects/motiva.png");
pub const OTTER: Asset = asset!("../assets/projects/otter.png");
pub const POUCHE: Asset = asset!("../assets/projects/pouche.png");
pub const TUIGREET: Asset = asset!("../assets/projects/tuigreet.png");
