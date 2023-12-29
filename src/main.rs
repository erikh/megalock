use anyhow::Result;
use megalock::{
    utils::{get_display, get_username},
    wm::Client,
};
use tracing::debug;

const PROGRAM_NAME: &str = "megalock";

fn get_tracing_level() -> tracing::Level {
    if std::env::var("TRACE").map_or(false, |x| !x.is_empty()) {
        tracing::Level::TRACE
    } else if std::env::var("DEBUG").map_or(false, |x| !x.is_empty()) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    }
}

fn setup_tracing() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(get_tracing_level())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

fn main() -> Result<()> {
    setup_tracing()?;

    debug!(
        "Locking desktop for '{}', display '{}'",
        get_username()?,
        get_display()
    );

    let client = megalock::x11::Client::get_client(0)?;

    if let Ok(timer) = std::env::var("EXIT_TRAP") {
        let timer = timer.parse()?;
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::new(timer, 0));
            std::process::exit(0)
        });
    }

    let events = client.lock().unwrap().events();
    std::thread::spawn(move || {
        while let Ok(e) = events.lock().unwrap().recv() {
            debug!("Event: {:?}", e);
        }
    });

    client.lock().unwrap().select_focused_window()?;
    client
        .lock()
        .unwrap()
        .open_fullscreen_window(PROGRAM_NAME.to_string())?;
    client.lock().unwrap().grab_pointer_and_keyboard()?;
    client.lock().unwrap().unlock_sleep()?;
    client.lock().unwrap().redraw_screen()?;
    client.lock().unwrap().raise_window()?;
    client.lock().unwrap().loop_until_pam(PROGRAM_NAME)?;

    Ok(())
}
