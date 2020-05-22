#![recursion_limit = "512"]
use failure::Error;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yew::InputData;

pub struct Model {
    link: ComponentLink<Self>,
    ws: Option<WebSocketTask>,
    wss: WebSocketService,
    value: i64,
    access_id: String,
    access_key: String,
    instance_level: i64,
    level_name: String,
    server_data: String,
}

impl Model {
    fn get_level_name(&self) -> String {
        return String::from(&self.level_name);
    }
}

pub enum Msg {
    AddOne,
    CreatOne,
    Level(String),
    Connect,
    Received(Result<String, Error>), // data received from server
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            ws: None,
            wss: WebSocketService::new(),
            value: 0,
            access_id: String::from(""),
            access_key: String::from(""),
            instance_level: 2,
            level_name: String::from("Better"),
            server_data: "server data".to_owned(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        println!("msg");
        let mut ys = services::console::ConsoleService::new();

        match msg {
            Msg::Received(Ok(s)) => {
                self.server_data.push_str(&format!("{}\n", &s));
            }
            Msg::Received(Err(s)) => {
                self.server_data.push_str(&format!(
                    "Error when reading data from server: {}\n",
                    &s.to_string()
                ));
            }
            Msg::Connect => {
                // self.console.log("Connecting");
                // let cbout = self.link.send_back(|Json(data)| Msg::Received(data));
                // let cbnot = self.link.send_back(|input| {
                //     ConsoleService::new().log(&format!("Notification: {:?}", input));
                //     match input {
                //         WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Disconnected,
                //         _ => Msg::Ignore,
                //     }
                // });
                // if self.ws.is_none() {
                //     let task = self
                //         .wss
                //         .connect("ws://127.0.0.1:8080/ws/", cbout, cbnot.into());
                //     self.ws = Some(task);
                // }
                // true
            }
            Msg::AddOne => {
                ys.log("AddOne");
                self.value += 1;
                self.level_name = match self.instance_level {
                    1 => String::from("Good"),
                    2 => String::from("Better"),
                    3 => String::from("Best"),
                    _ => String::from("Bad Guy!"),
                };
            }
            Msg::CreatOne => {
                ys.log("CreatOne");
            }
            Msg::Level(i) => {
                ys.log(&format!("Level {}", i));
                match i.as_str() {
                    "1" => self.level_name = String::from("Good"),
                    "2" => self.level_name = String::from("Better"),
                    "3" => self.level_name = String::from("Best"),
                    _ => self.level_name = String::from("Bad Guy"),
                }
            }
        }
        ys.log("to return true");
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div
                    class="uk-card uk-card-default uk-card-body"
                    style="margin-bottom: 200px; z-index: 980;"
                    uk-sticky="animation: uk-animation-slide-top; bottom: #sticky-on-scroll-up"
                >
                    <h1 class="uk-heading-small uk-text-lead"> {"Tomato Workstation"} </h1>
                </div>
                <div class="uk-section">
                    <div class="uk-container uk-container-small" uk-sticky="uk-flex-top">
                        <div>
                            <div class="uk-card uk-card-default uk-card-small">
                                <div class="uk-text-center uk-card-body">
                                    {"Do you want a tomato after 1 hour work?"}<br /> {"Let's get details."}
                                    <br />
                                    {String::from(&self.level_name)}
                                </div>
                            </div>
                        </div>
                        <h1 class="uk-heading-divider"></h1>
                        // <div class="uk-card uk-card-default uk-card-hover uk-card-body">
                        //     <div class="uk-grid uk-child-width-1-1">
                        //         <div>
                        //             <div class="uk-text-center uk-card-body uk-margin-small-top">
                        //                 {"Input your Access ID and Key."}
                        //             </div>
                        //         </div>
                        //         <div>
                        //             <div class="uk-grid uk-child-width-1-2">
                        //                 <label class="uk-text-large">{"Access ID"}</label>
                        //                 <input class="uk-input uk-width-1-2" value={&self.access_id} type="text" placeholder="as;ldkf;lk" />
                        //             </div>
                        //         </div>
                        //         <div>
                        //             <div class="uk-grid uk-child-width-1-2">
                        //                 <label class="uk-text-large">{"Access Key"}</label>
                        //                 <input class="uk-input uk-width-1-2" value={&self.access_key} type="text" placeholder="as;ldkf;lk" />
                        //             </div>
                        //         </div>
                        //     </div>
                        // </div>
                        <h1 class="uk-heading-divider"></h1>
                        <div class="uk-card uk-card-default uk-card-hover uk-card-body uk-height-max-large">
                            <div class="uk-grid uk-child-width-1-2">
                                <label class="uk-text-large">{"Tech Stack"}</label>
                                <select class="uk-select">
                                    <option>{"Rust"}</option>
                                    <option>{"Golang"}</option>
                                </select>
                            </div>
                            <div class="uk-grid uk-child-width-1-2">
                                <label class="uk-text-large">
                                    {self.get_level_name()}
                                </label>
                                <input class="uk-range" oninput=self.link.callback(|i: InputData| Msg::Level(i.value)) type="range" min="1" max="3" step="1" />
                            </div>
                            <br />
                            // <div class="uk-grid">
                                <button class="uk-container-center uk-button uk-button-primary" onclick=self.link.callback(|_| Msg::CreatOne)>{ "+1" }</button>
                            // </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
