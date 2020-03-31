use crate::virtualdb::{Database, Status, Todo};
use wasm_bindgen::prelude::*;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;

const KEY: &'static str = "yew.tut.database";

#[wasm_bindgen(
    inline_js = "export function refreshform(form) { document.getElementById(form).reset(); }"
)]
extern "C" {
    fn refreshform(form: &str);
}

pub enum Msg {
    AddTodo,
    DoneTodo(usize),
    ArchiveTodo(usize),
    SetTitle(String),
    SetDescription(String),
}

pub struct App {
    link: ComponentLink<Self>,
    storage: StorageService,
    database: Database,
    temp_todo: Todo,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);
        let Json(database) = storage.restore(KEY);
        let database = database.unwrap_or_else(|_| Database::new());
        App {
            link,
            storage,
            database,
            temp_todo: Todo::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTodo => {
                if self.temp_todo.is_valid() {
                    self.temp_todo.status = Status::Active;
                    self.database.todos.push(self.temp_todo.clone());
                    self.storage.store(KEY, Json(&self.database));
                    self.temp_todo = Todo::new();
                    refreshform("newtodoform");
                }
            }
            Msg::DoneTodo(pos) => {
                self.database.todos.get_mut(pos).unwrap().status = Status::Completed;
                self.storage.store(KEY, Json(&self.database));
            }
            Msg::ArchiveTodo(pos) => {
                self.database.todos.get_mut(pos).unwrap().status = Status::Archived;
                self.storage.store(KEY, Json(&self.database));
            }
            Msg::SetTitle(title) => {
                self.temp_todo.title = title;
                return false;
            }
            Msg::SetDescription(description) => {
                self.temp_todo.description = description;
                return false;
            }
        }
        true
    }

    fn view(&self) -> Html {
        let render_item = |(idx, todo): (usize, &Todo)| match todo.status {
            Status::Archived => html! {},
            Status::Completed => html! {
                    <>
                        <div class="card">
                            <header>
                                <label>
                                    <button onclick=self.link.callback(move |_| Msg::ArchiveTodo(idx))>{ "Archive" }</button>
                                    <span>{ format!(" {}", todo.title) }</span>
                                </label>
                            </header>
                            <div class="card-body"><label>{ &todo.description }</label></div>
                        </div>
                    </>
            },
            _ => html! {
                <>
                    <div class="card">
                        <header>
                            <label>
                                <input type="checkbox" onclick=self.link.callback(move |_| Msg::DoneTodo(idx)) />
                                <span class="checkable">{ &todo.title }</span>
                            </label>
                        </header>
                        <div class="card-body"><label>{ &todo.description }</label></div>
                    </div>
                </>
            },
        };
        html! {
            <div>
                <h1>{"Todo List "}</h1>
                { for self.database.todos.iter().enumerate().map(render_item) }
                <div class="card">
                    <form id="newtodoform">
                        <label class="stack"><input placeholder="Title" oninput=self.link.callback(|e: InputData|  Msg::SetTitle(e.value)) /></label>
                        <label class="stack"><textarea rows=2 placeholder="Description" oninput=self.link.callback(|e: InputData|  Msg::SetDescription(e.value))></textarea></label>
                        <button class="stack icon-paper-plane" onclick=self.link.callback(|_|  Msg::AddTodo)>{ "Add Todo" }</button>
                    </form>
                </div>
            </div>
        }
    }
}
