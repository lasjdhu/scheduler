use leptos::*;

#[derive(Clone)]
pub struct TodoItem {
    pub id: u32,
    pub task: String,
    pub status: bool,
}

#[component]
pub fn TodoItem<F>(todo_item: TodoItem, delete_callback: F) -> impl IntoView
where
    F: Fn(u32) + 'static,
{
    let (status, set_status) = create_signal(todo_item.status);

    let on_click = move |_| set_status.update(|status| *status = !*status);

    let task_title_style =
        move || format!("text-sm font-semibold leading-6 text-gray-900 {}", if status() { "line-through" } else { "" });
    let complete_button_style = move || {
        format!(
            "rounded-md bg-indigo-600 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600 {} w-24",
        if !status() {
                "opacity-100"
            } else {
                "opacity-50"
            }
        )
    };

    view! {
        <ul role="list" class="divide-y divide-gray-100">
            <li class="flex justify-between gap-x-6 py-5">
                <div class="flex min-w-0 gap-x-4">
                    <div class="min-w-0 flex-auto">
                        <p class={task_title_style}>{todo_item.task}</p>
                    </div>
                </div>
                <div class="flex gap-x-4">
                    <button
                    on:click={on_click}
                    class={complete_button_style}
                >
                    {move || if !status() { "Complete" } else { "Undo" }}
                </button>

                <button
                    on:click={move |_| delete_callback(todo_item.id)}
                    class="px-3.5 py-2.5 text-sm font-semibold text-red-600"
                >
                    "Delete"
                </button>
                </div>
            </li>
        </ul>
    }
}
