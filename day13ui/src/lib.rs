#![recursion_limit = "256"]

use std::fmt;

use stdweb::web::{html_element::CanvasElement, CanvasRenderingContext2d};
use yew::{
    components::Select,
    html,
    services::{render::RenderTask, RenderService, Task},
    Callback, Component, ComponentLink, Html, NodeRef, ShouldRender,
};

use day13::{Event, Game, Tile, ISTRUCTIONS};
use intcode;

pub struct Model {
    render: RenderService,
    canvas_ref: NodeRef,
    coins: Option<Coins>,
    game: Option<Game>,
    task: Option<RenderTask>,
    callback: Callback<f64>,
    score: intcode::Memory,
}

pub enum Msg {
    Start,
    Reset,
    SelectCoins(Coins),
    Frame,
}

#[derive(PartialEq, Clone)]
pub enum Coins {
    Zero,
    Two,
}

impl fmt::Display for Coins {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Coins::Zero => fmt.write_str("zero"),
            Coins::Two => fmt.write_str("two"),
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), mut link: ComponentLink<Self>) -> Self {
        Self {
            render: RenderService::new(),
            canvas_ref: NodeRef::default(),
            coins: None,
            game: None,
            task: None,
            callback: link.send_back(|_| Msg::Frame),
            score: 0,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Start => {
                if let Some(canvas) = self.canvas_ref.try_into::<CanvasElement>() {
                    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

                    context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
                }

                self.game = Some(Game::new(
                    &ISTRUCTIONS,
                    if let Some(Coins::Two) = self.coins {
                        Some(2)
                    } else {
                        None
                    },
                ));

                self.score = 0;
                self.task = Some(self.render.request_animation_frame(self.callback.clone()));

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

                self.coins = None;
                self.game = None;
                self.score = 0;

                true
            }
            Msg::SelectCoins(coins) => {
                self.coins = Some(coins);

                true
            }
            Msg::Frame => {
                if let Some(game) = &mut self.game {
                    match game.play() {
                        Event::Halt => {
                            self.task = None;
                            self.game = None;
                        }
                        e => {
                            match e {
                                Event::Draw(tile, (x, y)) => {
                                    if let Some(canvas) =
                                        self.canvas_ref.try_into::<CanvasElement>()
                                    {
                                        let context: CanvasRenderingContext2d =
                                            canvas.get_context().unwrap();

                                        context.set_fill_style_color(match tile {
                                            Tile::Empty => "white",
                                            Tile::Block => "black",
                                            Tile::HorizontalPaddle => "blue",
                                            Tile::Ball => "red",
                                            Tile::Wall => "brown",
                                        });

                                        const SCALE_X: f64 = 10.0;
                                        const SCALE_Y: f64 = 20.0;

                                        context.fill_rect(
                                            x as f64 * SCALE_X,
                                            y as f64 * SCALE_Y,
                                            SCALE_X,
                                            SCALE_Y,
                                        );
                                    }
                                }
                                Event::Score(score) => {
                                    self.score = score;
                                }
                                _ => {}
                            }

                            self.task =
                                Some(self.render.request_animation_frame(self.callback.clone()));
                        }
                    }
                }

                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        let coins = vec![Coins::Zero, Coins::Two];

        html! {
            <div>
                <nav class="menu">
                <button disabled={self.task.is_some() || self.coins.is_none()} onclick=|_| Msg::Start>{ "Start" }</button>
                <button disabled=self.task.is_none() onclick=|_| Msg::Reset>{ "Reset" }</button>
                <Select<Coins> disabled=self.task.is_some() options=coins onchange=Msg::SelectCoins selected=self.coins.to_owned()/>
                <p>{ format!("Score: {}", self.score) }</p>
                </nav>
                <canvas ref=self.canvas_ref.clone() width="500" height="500"></canvas>
                </div>
        }
    }
}
