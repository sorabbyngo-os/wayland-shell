//! wayland-shell — Sorabbyngo OS AI-native Wayland compositor

mod compositor;
mod shell;
mod agents;
mod voice;
mod vm;
mod adaptive;
mod render;
mod l10n;

use anyhow::Result;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("wayland-shell starting");
    Ok(())
}
