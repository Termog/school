use yew::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use gloo::events::{EventListener,EventListenerOptions};
use web_sys;
use argon2;


//hardcoded password hash
const HASH: &str = "$argon2id$v=19$m=16,t=2,p=1$c2hIWXREQUhOWkk2aXlXUQ$Uc5HDNpXIrV/eR407fkvfw";

//all variants of messages
enum Msg {
    ChangeLockState,
    OnInput(String),
}

//structure that holds our pages state
struct Model {
    //if the page is locked or not
    locked: bool,
    //inputed password
    input_value: String,
    //event listener for keyboard input
    kbd_listener: Option<EventListener>,
}

//implementation of default functions for our state struct
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    //constructor
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            locked: true,
            kbd_listener: None,
            input_value: "".to_string(),
        }
    }

    //function that gets called on every page rerender
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        //match statement for our message
        match msg {
            //message issued by the lock unlock button
            Msg::ChangeLockState => {
                //match on the state of lock
                match self.locked {
                    true => {
                        //if the page is locked verify the password
                        let password = &self.input_value.clone();
                        let correct = argon2::verify_encoded(HASH, &password.as_bytes()).unwrap_throw();
                        self.input_value = "".to_string();
                        //if the password is correct unlock
                        if correct {
                            self.locked ^= true;
                        } 
                        true
                    }
                    //if the page is unlocked lock
                    false => {
                        self.locked ^= true;
                        true
                    }
                }
            },
            //update password value in apps state
            Msg::OnInput(value) => {
                self.input_value = value;
                true
            }
        }
    }

    //function that gets called on rerender
    fn rendered(&mut self, _ctx: &Context<Self>, _:bool) {
        let document = gloo::utils::document();
        let options = EventListenerOptions::enable_prevent_default();
       // if the page is locked add a keyboard event listener
        let kbd_listener = match self.locked {
            false => None,
            true => Some(EventListener::new_with_options(&document, "keydown",options,  move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            let keycode = event.key_code();
            //if Ctrl S or Ctrl U are pressed prevent default
            if  (event.ctrl_key() || event.meta_key()) && (keycode == 83 ||  keycode == 85) {
                event.prevent_default();
                event.stop_immediate_propagation();
            }})),
        };
        //udapte value of kbd_listener
        self.kbd_listener = kbd_listener;
    }


    //components scope
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let state = match self.locked {
            true => "unlock",
            false => "lock",
        };
        let input_type = match self.locked {
            true => "password",
            false => "hidden",
        };
        html! {
            <div id="page" locked={ format!("{}",self.locked) } >
                <div>
                    <input type={input_type} value={self.input_value.clone()} oninput={link.callback(|e: web_sys::InputEvent| {
                        let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                        Msg::OnInput( input.value() )
                    })} />
                    <button onclick={link.callback(|_| Msg::ChangeLockState)}>{ state }</button>
                </div>
                <div>
                    <h1>{"Lorem ipsum dolor sit amet"}</h1>
                    <div>
                        <p>{"Lorem ipsum dolor sit amet. Et iusto earum sed veritatis possimus vel quibusdam esse eum odio quae et fuga totam. Ut obcaecati consequatur sit nisi temporibus eos alias modi est quia sapiente et voluptatem reiciendis? Eos sint omnis a praesentium aliquam non earum quia sit nemo voluptatem est omnis labore quo laboriosam asperiores."}</p>
                        <p>{"Eos assumenda beatae et odit molestiae qui ratione recusandae non quam facilis. Aut assumenda nobis a velit voluptatum ea sequi earum. Ut suscipit quos vel voluptatem esse eum expedita veniam sed aliquam reiciendis aut doloremque reiciendis aut dicta magni. Non dolores quod aut nihil blanditiis sed nemo dolor ad praesentium dicta et autem modi!"}</p>
                        <p>{"Rem voluptate sint qui dolores deleniti ad quidem tenetur sed voluptatibus voluptatem cum deserunt magni. Et voluptate numquam et reiciendis veniam qui debitis expedita est porro quia non laboriosam voluptate est dolor assumenda non Quis cumque. Et error quae qui rerum necessitatibus ut ipsam dolores est consequatur dolores At nisi magnam qui sequi enim."}</p>
                    </div>
                </div>
            </div>
        }
    }
}
fn main() {
    yew::start_app::<Model>();
}
