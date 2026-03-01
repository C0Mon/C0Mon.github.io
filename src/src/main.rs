use leptos::mount::mount_to_body;
use src::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
