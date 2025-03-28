use hyprland::{async_closure, event_listener::AsyncEventListener};

#[tokio::main]
async fn main() -> hyprland::Result<()> {
    let mut event_listener = AsyncEventListener::new();

    event_listener.add_active_window_changed_handler(async_closure! { move |window_data| {
            if let Some(data) = window_data {
                println!("{} - {}: {}", chrono::Local::now().timestamp_millis(), data.class, data.title);
            }
        }
    });

    event_listener.start_listener_async().await
}
