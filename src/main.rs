use leptos::prelude::*;

#[derive(Clone)]
struct Task {
    text: String,
    completed: bool,
}

#[component]
fn App() -> impl IntoView {
    let (tasks, set_tasks) = signal(Vec::<Task>::new());
    let (new_task, set_new_task) = signal(String::new());

    view! {
        <main>
            <h1>"Task Manager"</h1>

            <input type="text" placeholder="Inserisci una task" prop:value=new_task
            on:input=move |ev| {
                set_new_task.set(event_target_value(&ev));
            }/>
            <button on:click=move |_| {
                if !new_task.get().trim().is_empty() {
                    set_tasks.update(|tasks| {
                        tasks.push(Task {
                            text: new_task.get(),
                            completed: false,
                        });
                    });
                    set_new_task.set(String::new());
                }
            }>"Aggiungi"</button>

            <ul>
                {move || {
                    tasks.get().into_iter()
                    .enumerate()
                    .map(|(index, task)| view! {
                        <li>
                        <input type="checkbox" prop:checked=task.completed
                        on:change=move |_| {
                            set_tasks.update(|tasks| {
                                tasks[index].completed = !tasks[index].completed;
                            })
                        }/>

                        <span style: text-decoration=if task.completed { "line-through" } 
                        else { "none" }>{task.text}</span>

                        <button on:click=move |_| {
                            set_tasks.update(|tasks| {
                                tasks.remove(index);
                            });
                        }> "Elimina"</button>
                        </li>
                    })
                    .collect_view()
                }}
            </ul>
        </main>
    }
}

fn main() {
    mount_to_body(App);
}