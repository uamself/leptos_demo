use leptos::*;
fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button class="border-spacing-2 bg-gray-100 border-2 border-gray-200 p-2 rounded-md"
            on:click=move |_| {
                // on stable, this is set_count.set(3);
                //set_count.set(3);
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            // on stable, this is move || count.get();
            // {move || count.get()}    相应式交互
            {count} //相应式交互
            // {count.get()} 非响应式
        </button>
    }
}
