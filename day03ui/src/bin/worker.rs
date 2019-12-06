#[cfg(feature = "native-worker")]
use yew::agent::Threaded;

fn main() {
    yew::initialize();
    #[cfg(feature = "native-worker")]
    day03ui::worker::Worker::register();
    yew::run_loop();
}
