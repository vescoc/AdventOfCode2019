extern crate day03;

use day03::*;
use std::collections::HashSet;
use stdweb::{
    traits::*,
    unstable::TryInto,
    web::{document, html_element::CanvasElement, CanvasRenderingContext2d},
};
use yew::{html, services::ConsoleService, Component, ComponentLink, Html, ShouldRender};

const OFFSET_X: f64 = 250.0;
const OFFSET_Y: f64 = 250.0;
const SCALE: f64 = 50.0;

pub struct Model {
    console: ConsoleService,
    visible: bool,
    part_1: u32,
    part_2: u32,
}

pub enum Msg {
    Start,
    Reset,
}

impl Model {
    fn draw_path(&self, context: &CanvasRenderingContext2d, path: &HashSet<Point>, color: &str) {
        context.set_fill_style_color(color);
        for p in path {
            context.fill_rect(
                OFFSET_X - p.x() as f64 / SCALE,
                OFFSET_Y - p.y() as f64 / SCALE,
                1.,
                1.,
            );
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), _: ComponentLink<Self>) -> Self {
        Self {
            console: ConsoleService::new(),
            visible: false,
            part_1: 0,
            part_2: 0,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Start => {
                self.part_1 = part_1();
                self.part_2 = part_2();

                self.visible = true;

                self.console.log(&format!("part_1: {}", self.part_1));
                self.console.log(&format!("part_2: {}", self.part_2));

                let canvas: CanvasElement = document()
                    .query_selector("#canvas")
                    .unwrap()
                    .expect("Didn't find canvas")
                    .try_into()
                    .unwrap();
                let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                let ((path1, _), (path2, _)) = (&DATA[0], &DATA[1]);

                context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

                context.set_fill_style_color("black");
                context.fill_rect(OFFSET_X, OFFSET_Y, 10.0, 10.0);

                self.draw_path(&context, path1, "blue");
                self.draw_path(&context, path2, "red");

                true
            }
            Msg::Reset => {
                self.visible = false;

                true
            }
        }
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
                <button onclick=|_| Msg::Start>{ "Start" }</button>
                <button disabled=!self.visible onclick=|_| Msg::Reset>{ "Reset" }</button>
                </nav>
            { data }
            <canvas id="canvas" style="border: 2px solid black" width="500" height="500"></canvas>
            </div>
        }
    }
}
