use leptos::*;
pub fn Latest() -> impl IntoView {
    view! {
     <div role="tablist" class="tabs tabs-boxed w-auto mt-2 ml-4">
      <a role="tab" class="tab">Tab 1</a>
      <a role="tab" class="tab tab-active">Tab 2</a>
      <a role="tab" class="tab">Tab 3</a>
    </div>

        }
}
