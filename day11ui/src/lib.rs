use std::fmt;

use stdweb::web::{html_element::CanvasElement, CanvasRenderingContext2d};
use yew::{
    html,
    services::ConsoleService,
    Component, ComponentLink, Html, NodeRef, ShouldRender,
    components::Select
};

pub struct Model {
    console: ConsoleService,
    canvas_ref: NodeRef,
    start_color: Option<Tile>,
}

pub enum Msg {
    Start,
    StartColor(Tile)
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

    fn create(_: (), _: ComponentLink<Self>) -> Self {
        Self {
            console: ConsoleService::new(),
            canvas_ref: NodeRef::default(),
            start_color: None,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::Start => {
                true
            }
            Msg::StartColor(tile) => {
                self.console.log(&format!("StartColor {}", tile));
                
                self.start_color = Some(tile);

                true
            }
        }
    }

    fn view(&self) -> Html<Self> {
        let start_colors = vec![Tile::White, Tile::Black];
        
        html! {
            <div>
                <nav class="menu">
                <button disabled=self.start_color.is_none() onclick=|_| Msg::Start>{ "Start" }</button>
                <Select<Tile> options=start_colors onchange=Msg::StartColor selected=self.start_color.to_owned()/>
                </nav>
                <canvas ref=self.canvas_ref.clone() style="border: 2px solid black;" width="500" height="500"></canvas>
                </div>
        }
    }
}
