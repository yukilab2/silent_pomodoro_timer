mod app;
mod settings;

use eframe::egui;
use app::PomodoroApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Pomodoro Timer")
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Pomodoro Timer",
        options,
        Box::new(|_cc| {
            let app = PomodoroApp::new();
            Ok(Box::new(app))
        }),
    )
}

