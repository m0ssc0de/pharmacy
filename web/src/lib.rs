#![recursion_limit = "512"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
    CreatOne,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            _ => println!("TODO"),
        }
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
                                </div>
                            </div>
                        </div>
                        <h1 class="uk-heading-divider"></h1>
                        <div class="uk-card uk-card-default uk-card-hover uk-card-body">
                            <div class="uk-grid uk-child-width-1-1">
                                <div>
                                    <div class="uk-text-center uk-card-body uk-margin-small-top">
                                        {"Input your Access ID and Key."}
                                    </div>
                                </div>
                                <div>
                                    <div class="uk-grid uk-child-width-1-2">
                                        <label class="uk-text-large">{"Access ID"}</label>
                                        <input class="uk-input uk-width-1-2" type="text" placeholder="as;ldkf;lk" />
                                    </div>
                                </div>
                                <div>
                                    <div class="uk-grid uk-child-width-1-2">
                                        <label class="uk-text-large">{"Access Key"}</label>
                                        <input class="uk-input uk-width-1-2" type="text" placeholder="as;ldkf;lk" />
                                    </div>
                                </div>
                            </div>
                        </div>
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
                                <label class="uk-text-large">{"Good"}</label>
                                <input class="uk-range" type="range" value="2" min="1" max="3" step="1" />
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
