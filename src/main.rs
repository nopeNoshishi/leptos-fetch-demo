use leptos::mount::mount_to_body;
use leptos_fetch_demo::example_app;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(example_app)
}
