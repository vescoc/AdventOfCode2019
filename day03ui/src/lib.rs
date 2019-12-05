extern crate day03;

use std::thread;

use stdweb::web::{html_element::CanvasElement, CanvasRenderingContext2d};
use yew::{
    html,
    worker::*,
    services::ConsoleService,
    Component, ComponentLink, Html, NodeRef, ShouldRender,
};

mod worker;

const OFFSET_X: f64 = 250.0;
const OFFSET_Y: f64 = 250.0;
const SCALE: f64 = 50.0;

pub struct Model {
    console: ConsoleService,
    worker: Box<dyn Bridge<worker::Worker>>,
    canvas_ref: NodeRef,
    part_1: Option<u32>,
    part_2: Option<u32>,
}

pub enum Msg {
    Start,
    Reset,
    Render,
    Worker(worker::Response),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), mut link: ComponentLink<Self>) -> Self {
        link.send_self(Msg::Render);
        
        let callback = link.send_back(Msg::Worker);
        let worker = worker::Worker::bridge(callback);
            
        Self {
            console: ConsoleService::new(),
            worker,
            part_1: None,
            part_2: None,
            canvas_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Start => {
                self.console.log(&format!("update thread {:?}", thread::current().id()));
                         
                self.worker.send(worker::Request::GetPart1);
                self.worker.send(worker::Request::GetPart2);

                true
            }
            Msg::Reset => {
                self.part_1 = None;
                self.part_2 = None;

                true
            }
            Msg::Render => {
                self.console.log(&format!("update thread {:?}", thread::current().id()));
                         
                if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                    context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

                    context.set_fill_style_color("black");
                    context.fill_rect(OFFSET_X, OFFSET_Y, 5.0, 5.0);
                }

                self.worker.send(worker::Request::GetWireInfos);
                
                false
            }
            Msg::Worker(worker::Response::WireInfo1(p)) => {
                if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                    context.set_fill_style_color("blue");
                    context.fill_rect(
                        OFFSET_X + p.x() as f64 / SCALE,
                        OFFSET_Y - p.y() as f64 / SCALE,
                        1.,
                        1.,
                    );
                }

                false
            }
            Msg::Worker(worker::Response::WireInfo2(p)) => {
                if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                    context.set_fill_style_color("red");
                    context.fill_rect(
                        OFFSET_X + p.x() as f64 / SCALE,
                        OFFSET_Y - p.y() as f64 / SCALE,
                        1.,
                        1.,
                    );
                }

                false
            }
            Msg::Worker(worker::Response::Part1(value)) => {
                self.part_1 = Some(value);
                
                true
            }
            Msg::Worker(worker::Response::Part2(value)) => {
                self.part_2 = Some(value);
                
                true
            }
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn view(&self) -> Html<Self> {
        let have_data = self.part_1.is_some() && self.part_2.is_some();
        let data = if have_data {
            html! {
                <ol>
                <li><p>{ self.part_1.unwrap() }</p></li>
                <li><p>{ self.part_2.unwrap() }</p></li>
                </ol>
            }
        } else {
            html! {}
        };

        html! {
            <div>
                <nav class="menu">
                <button disabled=have_data onclick=|_| Msg::Start>{ "Start" }</button>
                <button disabled=!have_data onclick=|_| Msg::Reset>{ "Reset" }</button>
                </nav>
            { data }
            <canvas ref=self.canvas_ref.clone() style="border: 2px solid black" width="500" height="500"></canvas>
            </div>
        }
    }
}
