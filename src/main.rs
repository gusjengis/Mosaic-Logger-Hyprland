use hyprland::{async_closure, event_listener::AsyncEventListener};

#[tokio::main]
async fn main() -> hyprland::Result<()> {
    let mut event_listener = AsyncEventListener::new();

    event_listener.add_active_window_changed_handler(
        async_closure! { move |data| println!("window changed to {data:#?}")},
    );

    event_listener.start_listener_async().await
}
