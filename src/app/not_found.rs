use leptos::prelude::*;

use crate::{
  components::{Card, Icon},
  icons::UNKNOWN_DOCUMENT,
};

#[component]
pub fn NotFound() -> impl IntoView {
  view! {
    <div class="flex justify-center py-16">
      <Card class="w-full max-w-[800px] text-center">
        <Icon icon=UNKNOWN_DOCUMENT size="3em" class="block mb-3" />

        <p class="mb-3 text-2xl text-accent">"You seem to have lost your way"</p>
        <p class="mt-2text-muted">"The page you are looking for does not exist."</p>
      </Card>
    </div>
  }
}
