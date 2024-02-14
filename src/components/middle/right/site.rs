use leptos::*;
pub fn Site() -> impl IntoView {
    view! {

    <div class="stats stats-vertical shadow w-96 mt-2">

      <div class="stat">
        <div class="stat-title">Downloads</div>
        <div class="stat-value">31K</div>
        <div class="stat-desc">Jan 1st - Feb 1st</div>
      </div>

      <div class="stat">
        <div class="stat-title">New Users</div>
        <div class="stat-value">4,200</div>
        <div class="stat-desc">up 400 (22%)</div>
      </div>

      <div class="stat">
        <div class="stat-title">New Registers</div>
        <div class="stat-value">1,200</div>
        <div class="stat-desc">down 90 (14%)</div>
      </div>

    </div>
        }
}
