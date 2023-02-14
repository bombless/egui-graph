use eframe::egui;
use shapes::Shapes;
use egui::Color32;

mod shapes;

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

const RED: Color32 = Color32::from_rgb(255, 0, 0);

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.draw_rectangle_lines(10., 10., 40., 40., 2., RED);

        });
    }
}
