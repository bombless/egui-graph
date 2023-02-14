use eframe::egui;
use egui::{Stroke, Rect, Pos2, Rounding, Color32};

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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            let min_pos = Pos2 { x: 10., y: 10. };
            let max_pos = Pos2 { x: 50., y: 50. };

            let rect = Rect { min: min_pos, max: max_pos };

            let rounding = Rounding::none();

            let stroke = Stroke {
                width: 2.,
                color: Color32::from_rgb(255, 0, 0),
            };
            
            ui.painter().rect_stroke(rect, rounding, stroke);

        });
    }
}
