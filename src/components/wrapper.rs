use leptos::*;

#[component]
pub fn Wrapper(children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col min-h-screen">
            {children()}
        </div>
    }
}
