use egui::{FontId, FontFamily};
use egui::{Rounding, Color32, Vec2};
use crate::random_grid;
use crate::a_star;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    value: f32,
    #[serde(skip)]
    array: [Color32; 17*17],
    #[serde(skip)]
    adjacency_list: Vec<Vec<a_star::Cell>>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            value: 1.0,
            array: random_grid::generate_grid(),
            adjacency_list: a_star::a_star_search(),         
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {value, array, adjacency_list} = self;

        let obstacles = a_star::get_obstacles(&adjacency_list);
        let path = a_star::get_path(&adjacency_list);
        // let explored = a_star::get_explored(&adjacency_list);
        let values = a_star::get_values(&adjacency_list);

        //for expl in explored {
        //    array[(expl[0] * 17) as usize + expl[1] as usize] = Color32::DARK_BLUE;
        //}

        for path_e in path {
            array[(path_e[0] * 17) as usize + path_e[1] as usize] = Color32::GOLD;
        }

        for obstacle in obstacles {
            array[(obstacle[0] * 17) as usize + obstacle[1] as usize] = Color32::DARK_GRAY;
        }

        array[0] = Color32::GREEN;
        array[17*17-1] = Color32::RED;

        let frame = egui::CentralPanel::default();
        frame.show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let mut _grid = egui::Grid::new("some_unique_id");
            let mut _new_grid = _grid.spacing(Vec2::new(5.0, 5.0));
            _new_grid = _new_grid.min_col_width(0.17);
            _new_grid.show(ui, |ui| {
                for x in 0..17 {
                    for y in 0..17 {
                        let (rect, _response) = ui.allocate_exact_size(Vec2::new(*value * 50.0, *value * 50.0), egui::Sense::click());
                        ui.painter().rect(
                            rect,
                            Rounding::default(),
                            array[x*17+y],
                            egui::Stroke::default(),
                        );
                        let f_val: String = values[y*17+x][0].to_string().chars().take(4).collect();
                        let g_val: String = values[y*17+x][1].to_string().chars().take(4).collect();
                        let h_val: String = values[y*17+x][2].to_string().chars().take(4).collect();
                        let x_coord: String = x.to_string();
                        let y_coord: String = y.to_string();
                        let coord = x_coord + "," + &y_coord;
                        ui.painter().text(rect.left_top(), egui::Align2::LEFT_TOP, f_val, FontId::new(9.0, FontFamily::Monospace), Color32::BLACK);
                        ui.painter().text(rect.right_top(), egui::Align2::RIGHT_TOP, g_val, FontId::new(9.0, FontFamily::Monospace), Color32::BLACK);
                        ui.painter().text(rect.left_bottom(), egui::Align2::LEFT_BOTTOM, h_val, FontId::new(9.0, FontFamily::Monospace), Color32::BLACK);
                        ui.painter().text(rect.right_bottom(), egui::Align2::RIGHT_BOTTOM, coord, FontId::new(9.5, FontFamily::Monospace), Color32::BLACK);
                        /*if _response.clicked() {
                            if array[x*17+y] == Color32::GRAY {
                                ui.painter().rect(
                                    rect,
                                    Rounding::default(),
                                    Color32::DARK_GRAY,
                                    egui::Stroke::default(),
                                );
                                array[x*17+y] = Color32::DARK_GRAY;
                                continue;
                            }
                            if array[x*17+y] == Color32::DARK_GRAY {
                                ui.painter().rect(
                                    rect,
                                    Rounding::default(),
                                    Color32::GRAY,
                                    egui::Stroke::default(),
                                );
                                array[x*17+y] = Color32::GRAY;
                            }
                        }*/
                    };
                    ui.end_row();
                }
            });
        });
    }
}