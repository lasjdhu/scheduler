use crate::components::navigation::Navigation;
use crate::components::todo_item::TodoItem;
use crate::components::wrapper::Wrapper;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (todo_items, set_todo_items) = create_signal(vec![
        TodoItem {
            id: 0,
            task: String::from("One"),
            status: false,
        },
        TodoItem {
            id: 1,
            task: String::from("Two"),
            status: false,
        },
        TodoItem {
            id: 2,
            task: String::from("Three"),
            status: true,
        },
        TodoItem {
            id: 3,
            task: String::from("Four"),
            status: false,
        },
    ]);

    let todo_task_input_ref: NodeRef<Input> = create_node_ref();
    let last_todo_id = move || todo_items().iter().map(|todo_item| todo_item.id).max();

    let delete_todo_item = move |todo_id: u32| {
        set_todo_items
            .update(move |todo_items| todo_items.retain(|todo_item| todo_item.id != todo_id))
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let new_task = todo_task_input_ref().expect("<input> to exist").value();
        let todo_id = last_todo_id().unwrap_or_default() + 1;

        if !new_task.is_empty() {
            set_todo_items.update(|items| {
                items.push(TodoItem {
                    id: todo_id,
                    task: new_task,
                    status: false,
                });
            });
            todo_task_input_ref()
                .expect("<input> to exist")
                .set_value("");
        }
    };

    view! {
        <Wrapper>
            <Navigation />
            <main class="flex flex-col lg:w-3/4 md:w-4/5 lg:m-auto md:m-auto px-4">
                <form class="flex pb-8 pt-4 gap-x-4" on:submit={on_submit}>
                    <input type="text" name="task" id="task" node_ref={todo_task_input_ref} class="block w-full rounded-md border-0 py-1.5 pl-7 pr-20 text-gray-900 ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" placeholder="Add a new task" />
                    <input class="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600" type="submit" value="Submit" />
                </form>

                <For
                   each={move || {
                        let mut todo_cloned = todo_items().clone();
                        todo_cloned.reverse();
                        todo_cloned
                    }}
                    key=|task| task.id
                    children=move |task: TodoItem| {
                        view! {
                            <TodoItem todo_item={task} delete_callback={delete_todo_item} />
                        }
                    }
                />
            </main>
        </Wrapper>
    }
}
