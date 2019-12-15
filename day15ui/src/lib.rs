#![recursion_limit = "256"]

use std::fmt;

use stdweb::web::{html_element::CanvasElement, CanvasRenderingContext2d};
use yew::{
    components, html,
    services::{render::RenderTask, ConsoleService, RenderService, Task},
    Callback, Component, ComponentLink, Html, NodeRef, ShouldRender,
};

use day15::{search, PROGRAM};

#[derive(PartialEq, Clone)]
pub enum Mode {
    SearchOxygenSystem,
    FillOxygen,
}

impl fmt::Display for Mode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Mode::SearchOxygenSystem => fmt.write_str("Searching Oxygen System"),
            Mode::FillOxygen => fmt.write_str("Filling Oxygen"),
        }
    }
}

pub struct Model {
    console: ConsoleService,
    render: RenderService,
    canvas_ref: NodeRef,
    mode: Option<Mode>,
    search: Option<search::Search>,
    task: Option<RenderTask>,
    callback: Callback<f64>,
}

impl Model {
    fn clear_canvas(&self) {
        if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
            let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

            context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
        }
    }
}

pub enum Msg {
    Start,
    Reset,
    SelectMode(Mode),
    Frame,
}

const SCALE: f64 = 10.;

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), mut link: ComponentLink<Self>) -> Self {
        Self {
            console: ConsoleService::new(),
            render: RenderService::new(),
            canvas_ref: NodeRef::default(),
            mode: None,
            search: None,
            task: None,
            callback: link.send_back(|_| Msg::Frame),
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Start => {
                if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                    self.clear_canvas();

                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                    match self.mode {
                        Some(Mode::SearchOxygenSystem) => {
                            context.set_fill_style_color("aqua");
                            context.fill_rect(250.0, 250.0, SCALE, SCALE);

                            self.search = Some(search::Search::new(&PROGRAM, |v| {
                                v == search::Tile::OxygenSystem
                            }))
                        }
                        Some(Mode::FillOxygen) => {
                            let mut search =
                                search::Search::new(&PROGRAM, |v| v == search::Tile::OxygenSystem);
                            loop {
                                match search.step() {
                                    Ok(search::Step::Found((x, y), _, cpu)) => {
                                        context.set_fill_style_color("red");
                                        context.fill_rect(
                                            250.0 + x as f64 * SCALE,
                                            250.0 + y as f64 * SCALE,
                                            SCALE,
                                            SCALE,
                                        );

                                        self.search =
                                            Some(search::Search::new_from_cpu(cpu, (x, y), |_| {
                                                false
                                            }));
                                        break;
                                    }
                                    Ok(_) => {}
                                    Err(v) => self.console.error(&format!("got error: {:?}", v)),
                                }
                            }
                        }
                        None => {}
                    }

                    if self.search.is_some() {
                        self.task =
                            Some(self.render.request_animation_frame(self.callback.clone()));
                    }
                }

                true
            }
            Msg::Reset => {
                self.clear_canvas();

                if let Some(mut task) = self.task.take() {
                    task.cancel()
                }

                self.mode = None;
                self.search = None;

                true
            }
            Msg::SelectMode(mode) => {
                self.mode = Some(mode);

                true
            }
            Msg::Frame => {
                if let Some(search) = &mut self.search {
                    match search.step() {
                        Ok(search::Step::Searching(found, searching)) => {
                            if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                                let context: CanvasRenderingContext2d =
                                    canvas.get_context().unwrap();

                                found.into_iter().for_each(|((x, y), tile)| {
                                    context.set_fill_style_color(match tile {
                                        search::Tile::Wall => "brown",
                                        search::Tile::Empty => "white",
                                        search::Tile::OxygenSystem => "lime",
                                    });

                                    context.fill_rect(
                                        250.0 + x as f64 * SCALE,
                                        250.0 + y as f64 * SCALE,
                                        SCALE,
                                        SCALE,
                                    );
                                });

                                context.set_fill_style_color("yellow");
                                searching.into_iter().for_each(|(x, y)| {
                                    context.fill_rect(
                                        250.0 + x as f64 * SCALE,
                                        250.0 + y as f64 * SCALE,
                                        SCALE,
                                        SCALE,
                                    );
                                });
                            }

                            self.task =
                                Some(self.render.request_animation_frame(self.callback.clone()));
                        }
                        Ok(search::Step::Found((x, y), _, _)) => {
                            if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                                let context: CanvasRenderingContext2d =
                                    canvas.get_context().unwrap();

                                context.set_fill_style_color("red");
                                context.fill_rect(
                                    250.0 + x as f64 * SCALE,
                                    250.0 + y as f64 * SCALE,
                                    SCALE,
                                    SCALE,
                                );
                            }
                            self.task = None;
                            self.search = None;
                        }
                        Err(v) => {
                            self.console
                                .error(&format!("got error from search: {:?}", v));
                            self.task = None;
                            self.search = None;
                        }
                    }
                }

                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        let modes = vec![Mode::SearchOxygenSystem, Mode::FillOxygen];

        html! {
            <div>
                <nav class="menu">
                <button disabled={self.task.is_some() || self.mode.is_none()} onclick=|_| Msg::Start>{ "Start" }</button>
                <button disabled=self.task.is_none() onclick=|_| Msg::Reset>{ "Reset" }</button>
                <components::Select<Mode> disabled=self.task.is_some() options=modes onchange=Msg::SelectMode selected=self.mode.to_owned()/>
                </nav>
                <canvas ref=self.canvas_ref.clone() width="500" height="500" style="border: 2px solid black;"></canvas>
                </div>
        }
    }
}
