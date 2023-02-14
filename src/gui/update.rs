use super::shapes::Shapes;
use super::MyApp;
use egui::Color32;

const RED: Color32 = Color32::from_rgb(255, 0, 0);

const CELL_WIDTH: f32 = 30.;
const CELL_HEIGHT: f32 = 30.;
const CELL_THIKNESS: f32 = 1.;
const CELL_PADDING: f32 = 3.;
const BAR_HEIGHT: f32 = 30.;
const LEFT_MARGIN: f32 = 30.;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let m = self.values.len();
        let n = self.values[0].len();

        egui::CentralPanel::default().show(ctx, |ui| {
            for i in 0 .. m {
                for j in 0 .. n {
                    let x = i as f32 * CELL_WIDTH + LEFT_MARGIN;
                    let y = j as f32 * CELL_HEIGHT + BAR_HEIGHT;

                    ui.draw_rectangle_lines(
                        x,
                        y,
                        CELL_WIDTH + CELL_THIKNESS,
                        CELL_HEIGHT + CELL_THIKNESS,
                        CELL_THIKNESS * 2.,
                        RED
                    );
                }
            }
        });
    }
}
