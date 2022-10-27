use std::collections::VecDeque;
use std::iter;

use yew::{html, Component, Context, Html, NodeRef};

use gloo_console as console;

use web_sys::HtmlInputElement;

pub struct Model {
    cpu: intcode::CPU,
    state: Option<Result<intcode::Run, intcode::Error>>,
    output: String,
    input: VecDeque<intcode::Memory>,
    command_ref: NodeRef,
}

pub enum Msg {
    Start,
    Command(String),
}

impl Model {
    fn run(&mut self) {
        console::log!("run");
        if self.state.is_none() {
            self.state = Some(self.cpu.run());
        }

        while let Some(ref state) = self.state {
            match state {
                Ok(intcode::Run::Output(c)) => {
                    self.output.push(u8::try_from(*c).unwrap() as char);
                    self.state = Some(self.cpu.run());
                }
                Ok(intcode::Run::NeedInput) => {
                    if let Some(c) = self.input.pop_front() {
                        self.cpu.set_input(Some(c));
                        self.state = Some(self.cpu.run());
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn init_cpu() -> intcode::CPU {
        intcode::CPU::new(intcode::parse(include_str!("../data.txt")), 0, None)
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        console::log!("create");
        let mut r = Self {
            cpu: Self::init_cpu(),
            state: None,
            input: VecDeque::new(),
            output: String::new(),
            command_ref: NodeRef::default(),
        };
        r.run();
        r
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                self.cpu = Self::init_cpu();
                self.state = None;
                self.output.clear();
                self.input.clear();
                
                self.run();

                true
            }
            Msg::Command(value) => {
                console::log!("command", &value);

                self.output.clear();
                self.input.clear();

                for c in value.chars().chain(iter::once('\n')) {
                    self.input.push_back(c as intcode::Memory);
                }

                self.run();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let command_ref = self.command_ref.clone();

        let onchange = link.batch_callback(move |_| {
            let command = command_ref.cast::<HtmlInputElement>();
            command.map(|command| {
                let value = command.value();
                command.set_value("");
                Msg::Command(value)
            })
        });

        let disabled = match self.state {
            None | Some(Ok(intcode::Run::Halt)) => false,
            _ => true,
        };

        let state = match self.state {
            Some(Ok(intcode::Run::NeedInput)) => "Waiting command...",
            Some(Ok(intcode::Run::Halt)) => "Halted!",
            Some(Ok(intcode::Run::Output(_))) => "Output...",
            Some(Err(_)) => "Error!",
            None => "Not started!",
        };

        html! {
            <>
                <button disabled={disabled} onclick={link.callback(|_| Msg::Start)}>{ "Start" }</button>
                <label for="command">
                    { "Command: " }
                    <input ref={self.command_ref.clone()}
                      disabled={!disabled}
                      {onchange}
                      id="command"
                      type="text" />
                </label>
                <p>{ state }</p>
                <p class="output"><pre id="display">{ &self.output }</pre></p>
            </>
        }
    }
}
