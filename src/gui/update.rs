use super::{shapes::Shapes, text::Text};
use super::MyApp;
use egui::{Color32, PointerButton, InputState, Pos2};
use egui::containers::Frame;

const CELL_WIDTH: f32 = 30.;
const CELL_HEIGHT: f32 = 30.;
const CELL_THIKNESS: f32 = 1.;
const BAR_HEIGHT: f32 = 30.;
const LEFT_MARGIN: f32 = 30.;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let m = self.values.len();
        let n = self.values[0].len();

        let options = Frame {
            fill: Color32::GRAY,
            ..Frame::default()
        };

        egui::CentralPanel::default().frame(options).show(ctx, |ui| {
            fn clicked_pos(i: &InputState) -> Option<Pos2> {
                if i.pointer.button_pressed(PointerButton::Primary) {
                    return i.pointer.interact_pos()
                }
                return None;
            }
            if let Some(Pos2 { x, y }) = ctx.input(clicked_pos) {
                let x_count = (x - LEFT_MARGIN) / CELL_WIDTH;
                let y_count = (y - BAR_HEIGHT) / CELL_HEIGHT;
                if x_count >= 0. && x_count < m as f32 && y_count >= 0. && y_count < n as f32 {
                    self.green_cells.clear();
                    self.green_cells.insert((x_count as usize, y_count as usize));
                }
            }

            for i in 0 .. m {
                for j in 0 .. n {
                    let x = i as f32 * CELL_WIDTH + LEFT_MARGIN;
                    let y = j as f32 * CELL_HEIGHT + BAR_HEIGHT;

                    if self.green_cells.contains(&(i, j)) {
                        ui.draw_rectangle(
                            x,
                            y,
                            CELL_WIDTH + CELL_THIKNESS,
                            CELL_HEIGHT + CELL_THIKNESS,
                            Color32::GREEN,
                        );
                    }

                    ui.draw_rectangle_lines(
                        x,
                        y,
                        CELL_WIDTH + CELL_THIKNESS,
                        CELL_HEIGHT + CELL_THIKNESS,
                        CELL_THIKNESS * 2.,
                        Color32::RED,
                    );

                    ui.draw_text(
                        &self.values[i][j].to_string(),
                        x + CELL_WIDTH / 2.,
                        y + CELL_HEIGHT / 2.,
                        18.,
                        Color32::BLACK,
                    );
                }
            }
        });
    }
}
