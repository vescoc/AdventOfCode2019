use yew::{
    html, Callback, Component, ComponentLink, Html, ShouldRender,
    services::{ConsoleService},
};

use day25;

pub struct Model {
    console: ConsoleService,
}

pub enum Msg {
    
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: (), mut link: ComponentLink<Self>) -> Self {
	Self {
	    console: ConsoleService::new(),
	}
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
	match msg {

	}
    }

    fn view(&self) -> Html<Self> {
	html! {
	    <div>
		</div>
	}
    }
}
