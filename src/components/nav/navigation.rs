use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {

    //导航开始部分
    <div class="navbar border bg-gray-50">
      <div class="navbar-start">
        <a class="btn btn-ghost text-xl">daisyUI开源博客</a>
      </div>
    //导航结束部分
      <div class="navbar-end">
        <ul class="menu menu-horizontal px-1">
          <li><a>首页</a></li>
          <li><a>分类</a></li>
          <li><a>其他</a></li>
        </ul>
        <button class="btn btn-ghost btn-circle">
          <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
        </button>
      </div>
    </div>    }
}
