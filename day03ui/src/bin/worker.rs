use yew::agent::Threaded;

fn main() {
    yew::initialize();
    day03ui::worker::Worker::register();
    yew::run_loop();
}
