use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
    <div class="join">
      <input class="join-item btn btn-square" type="radio" name="options" aria-label="1" checked />
      <input class="join-item btn btn-square" type="radio" name="options" aria-label="2" />
      <input class="join-item btn btn-square" type="radio" name="options" aria-label="3" />
      <input class="join-item btn btn-square" type="radio" name="options" aria-label="4" />
    </div>
    }
}
