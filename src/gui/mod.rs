use eframe::egui;

mod shapes;
mod update;

pub fn run(ranks: Vec<Vec<i32>>, values: Vec<Vec<i32>>) {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new(ranks, values))),
    ).unwrap()
}

struct MyApp {
    ranks: Vec<Vec<i32>>,
    values: Vec<Vec<i32>>,
}

impl MyApp {
    fn new(ranks: Vec<Vec<i32>>, values: Vec<Vec<i32>>) -> MyApp {
        Self {
            ranks,
            values,
        }
    }
}
