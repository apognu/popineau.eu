fn main() {
  println!("cargo:rerun-if-changed=src");
  println!("cargo:rerun-if-changed=assets/css");

  topcoat::tailwind::BuildConfig::new().input("assets/css/app.css").render().unwrap();
  topcoat::icon::iconify::BuildConfig::new().icon_set("material-symbols-light").icon_set("fa7-brands").stage().unwrap();
}
