extern crate day03;

use day03::*;
use stdweb::web::{html_element::CanvasElement, CanvasRenderingContext2d};
use yew::{
    html,
    services::{render::RenderTask, ConsoleService, RenderService},
    Callback, Component, ComponentLink, Html, NodeRef, ShouldRender,
};

const OFFSET_X: f64 = 250.0;
const OFFSET_Y: f64 = 250.0;
const SCALE: f64 = 50.0;

pub struct Model {
    console: ConsoleService,
    render: RenderService,
    visible: bool,
    part_1: u32,
    part_2: u32,
    canvas_ref: NodeRef,
    render_task: Option<RenderTask>,
    render_callback: Callback<f64>,
    path_1: Vec<&'static Point>,
    path_2: Vec<&'static Point>,
    index: usize,
}

pub enum Msg {
    Start,
    Reset,
    Render,
    Frame,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), mut link: ComponentLink<Self>) -> Self {
        link.send_self(Msg::Render);
        let mut r = Self {
            console: ConsoleService::new(),
            render: RenderService::new(),
            visible: false,
            part_1: 0,
            part_2: 0,
            canvas_ref: NodeRef::default(),
            render_task: None,
            render_callback: link.send_back(|_| Msg::Frame),
            path_1: DATA[0].0.iter().collect(),
            path_2: DATA[1].0.iter().collect(),
            index: 0,
        };

        r.console.log(&format!("path_1 len {}", r.path_1.len()));
        r.console.log(&format!("path_2 len {}", r.path_2.len()));

        r
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Start => {
                self.part_1 = part_1();
                self.part_2 = part_2();

                self.visible = true;

                self.console.log(&format!("part_1: {}", self.part_1));
                self.console.log(&format!("part_2: {}", self.part_2));

                true
            }
            Msg::Reset => {
                self.visible = false;

                true
            }
            Msg::Render => {
                if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                    context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

                    context.set_fill_style_color("black");
                    context.fill_rect(OFFSET_X, OFFSET_Y, 5.0, 5.0);

                    self.render_task = Some(
                        self.render
                            .request_animation_frame(self.render_callback.clone()),
                    );
                }

                false
            }
            Msg::Frame => {
                if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                    if let Some(p) = self.path_1.get(self.index) {
                        context.set_fill_style_color("blue");
                        context.fill_rect(
                            OFFSET_X + p.x() as f64 / SCALE,
                            OFFSET_Y - p.y() as f64 / SCALE,
                            1.,
                            1.,
                        );
                    }

                    if let Some(p) = self.path_2.get(self.index) {
                        context.set_fill_style_color("red");
                        context.fill_rect(
                            OFFSET_X + p.x() as f64 / SCALE,
                            OFFSET_Y - p.y() as f64 / SCALE,
                            1.,
                            1.,
                        );
                    }

                    self.index += 1;

                    if self.index < self.path_1.len() || self.index < self.path_2.len() {
                        if self.index % 1000 == 0 && self.index != 0 {
                            self.console.log(&format!("Frame: {}", self.index));
                        }

                        self.render_task = Some(
                            self.render
                                .request_animation_frame(self.render_callback.clone()),
                        );
                    } else {
                        self.console.log(&format!("Frame done: {}", self.index));
                    }
                }

                false
            }
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        false
    }

    fn view(&self) -> Html<Self> {
        let data = if self.visible {
            html! {
                <ol>
                <li><p>{ format!("{:?}", self.part_1) }</p></li>
                <li><p>{ format!("{:?}", self.part_2) }</p></li>
                </ol>
            }
        } else {
            html! {}
        };

        html! {
            <div>
                <nav class="menu">
                <button disabled=self.visible onclick=|_| Msg::Start>{ "Start" }</button>
                <button disabled=!self.visible onclick=|_| Msg::Reset>{ "Reset" }</button>
                </nav>
            { data }
            <canvas ref=self.canvas_ref.clone() style="border: 2px solid black" width="500" height="500"></canvas>
            </div>
        }
    }
}
