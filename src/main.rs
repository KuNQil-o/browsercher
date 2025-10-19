mod app;
mod args;
mod config;
mod context;
mod lifecycle;
mod tests;

use crate::app::BrowsercherApp;
use crate::lifecycle::LifecycleManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = BrowsercherApp::new();
    LifecycleManager::new(app).run()
}
