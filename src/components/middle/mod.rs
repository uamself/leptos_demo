pub mod left;
pub mod right;

use left::carousel::Carousel;
use left::content::Content;
use left::page::Page;
use leptos::*;
use right::about_me::AboutMe;
use right::label::Lable;
use right::latest::Latest;
use right::site::Site;

#[component]
pub fn Middle() -> impl IntoView {
    view! {
    <div class="flex h-auto border bg-gray-600">
        <div class="h-auto w-2/3 border">
          <Carousel/>
          <Content/>
          <Content/>
          <Content/>
          <Page/>
        </div>
        <div class="h-auto w-1/3 border">
          <AboutMe/>
          <Lable/>
          <Latest/>
          <Site/>
        </div>
    </div>
    }
}
