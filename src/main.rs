use leptos::*;
use leptos_demo::components::{footer::Footer, middle::Middle, nav::navigation::Nav};
fn main() {
    leptos::mount_to_body(|| {
        view! {
            <Nav/>
            <Middle/>
            <Footer/>
        }
    })
}

#[component]
fn Div() -> impl IntoView {
    view! {

    <div class="navbar bg-neutral text-neutral-content">
         <div class="flex-1">
           <a class="btn btn-ghost text-xl">daisyUI 开源博客</a>
         </div>
         <div class="flex-none gap-2">
           <div class="form-control">
             <input
               type="text"
               placeholder="Search"
               class="input input-bordered w-24 md:w-auto"
             />
           </div>
           <div class="dropdown dropdown-end">
             <button class="btn btn-ghost btn-circle">
               <svg
                 xmlns="http://www.w3.org/2000/svg"
                 class="h-5 w-5"
                 fill="none"
                 viewBox="0 0 24 24"
                 stroke="currentColor"
               >
                 <path
                   stroke-linecap="round"
                   stroke-linejoin="round"
                   stroke-width="2"
                   d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                 />
               </svg>
             </button>

           </div>
         </div>
       </div>
    }
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
