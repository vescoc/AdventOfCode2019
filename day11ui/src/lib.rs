#![recursion_limit = "256"]

use std::fmt;

use stdweb::web::{html_element::CanvasElement, CanvasRenderingContext2d};
use yew::{
    components::Select,
    html,
    services::{render::RenderTask, ConsoleService, RenderService, Task},
    Callback, Component, ComponentLink, Html, NodeRef, ShouldRender,
};

use day11::{simple::Painter, DATA};

pub struct Model {
    console: ConsoleService,
    render: RenderService,
    canvas_ref: NodeRef,
    start_color: Option<Tile>,
    painter: Option<Painter>,
    task: Option<RenderTask>,
    render_callback: Callback<f64>,
}

pub enum Msg {
    Start,
    Reset,
    StartColor(Tile),
    Render,
}

#[derive(PartialEq, Clone)]
pub enum Tile {
    White,
    Black,
}

impl fmt::Display for Tile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Tile::White => fmt.write_str("white"),
            Tile::Black => fmt.write_str("black"),
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), mut link: ComponentLink<Self>) -> Self {
        Self {
            console: ConsoleService::new(),
            render: RenderService::new(),
            canvas_ref: NodeRef::default(),
            start_color: None,
            painter: None,
            task: None,
            render_callback: link.send_back(|_| Msg::Render),
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Start => {
                let mut painter = Painter::new(&DATA);
                match self.start_color {
                    Some(Tile::White) => {
                        painter.insert((0, 0), 0);
                    }
                    Some(Tile::Black) => {
                        painter.insert((0, 0), 1);
                    }
                    _ => {}
                }

                self.painter = Some(painter);

                self.task = Some(
                    self.render
                        .request_animation_frame(self.render_callback.clone()),
                );

                true
            }
            Msg::Reset => {
                if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                    context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
                }

                if let Some(mut task) = self.task.take() {
                    task.cancel();
                }

                self.start_color = None;
                self.painter = None;
                self.task = None;

                true
            }
            Msg::StartColor(tile) => {
                self.console.log(&format!("StartColor {}", tile));

                self.start_color = Some(tile);

                true
            }
            Msg::Render => {
                self.console.log("render...");

                if let Some(painter) = &mut self.painter {
                    if let Some(((x, y), value)) = painter.next() {
                        if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                            let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                            context.set_fill_style_color(match value {
                                0 => "yellow",
                                1 => "black",
                                _ => "white",
                            });

                            const SCALE: f64 = 5.0;

                            context.fill_rect(
                                250.0 + x as f64 * SCALE,
                                250.0 + y as f64 * SCALE,
                                SCALE,
                                SCALE,
                            );
                        }

                        self.task = Some(
                            self.render
                                .request_animation_frame(self.render_callback.clone()),
                        );
                    }
                }

                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        let start_colors = vec![Tile::White, Tile::Black];

        html! {
            <div>
                <nav class="menu">
                <button disabled={self.start_color.is_none() && self.painter.is_none()} onclick=|_| Msg::Start>{ "Start" }</button>
                <button disabled=self.task.is_none() onclick=|_| Msg::Reset>{ "Reset" }</button>
                <p>{ "Start Color: " }<Select<Tile> disabled=self.task.is_some() options=start_colors onchange=Msg::StartColor selected=self.start_color.to_owned()/></p>
                </nav>
                <canvas ref=self.canvas_ref.clone() style="border: 2px solid black;" width="500" height="500"></canvas>
                </div>
        }
    }
}
