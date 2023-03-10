use eframe::egui;
use egui::{FontFamily, FontDefinitions, FontData};
use cell_highlight::Highlight;
use crate::solution::UnionFind;

mod shapes;
mod text;
mod update;
mod cell_highlight;


#[derive(PartialEq)]
enum Tab {
    Ranks,
    Values,
    Expected,
}

pub fn run(ranks: Vec<Vec<i32>>, values: Vec<Vec<i32>>, expected: Vec<Vec<i32>>, uf: UnionFind<(usize, usize)>) {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert("my_font".to_owned(),
    FontData::from_static(include_bytes!("../../LoftygoalsRegular-9Y5Xy.otf"))); // .ttf and .otf supported

    // egui_ctx.set_fonts(fonts);

    // Put my font first (highest priority):
    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(900.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new(ranks, values, expected, uf))),
    ).unwrap()
}

struct MyApp {
    ranks: Vec<Vec<i32>>,
    values: Vec<Vec<i32>>,
    expected: Vec<Vec<i32>>,
    green_cells: Highlight,
    tab: Tab,
    pair: Option<(usize, usize)>,
}

impl MyApp {
    fn new(ranks: Vec<Vec<i32>>, values: Vec<Vec<i32>>, expected: Vec<Vec<i32>>, uf: UnionFind<(usize, usize)>) -> MyApp {
        Self {
            ranks,
            values,
            expected,
            green_cells: Highlight::new(uf),
            tab: Tab::Ranks,
            pair: None,
        }
    }
}
