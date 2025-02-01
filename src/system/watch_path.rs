use std::path::Path;

use futures::{channel::mpsc, Stream};

#[allow(unused)]
/**
 * Used to regenerate static pages
 * Cf. https://github.com/leptos-rs/leptos/tree/main/examples/static_routing
 */
pub fn watch_path(path: &Path) -> impl Stream<Item = ()> {
    #[allow(unused)]
    let (mut producer, consumer) = mpsc::channel(0);

    #[cfg(feature = "ssr")]
    {
        use notify::{RecursiveMode, Watcher};

        let mut watcher =
            notify::recommended_watcher(move |event: Result<_, _>| {
                if event.is_ok() {
                    // If this fails, it's because the buffer is full
                    // this means we've already notified before it's regenerated,
                    // so this page will be queued for regeneration already
                    _ = producer.try_send(());
                }
            })
            .expect("Could not create watcher");

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher
            .watch(path, RecursiveMode::NonRecursive)
            .expect("Could not watch path");

        // We want this to run as long as the server is alive
        std::mem::forget(watcher);
    }

    consumer
}
