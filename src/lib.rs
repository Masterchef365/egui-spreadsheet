use egui::{Color32, Id, Pos2, Rect, ScrollArea, Sense, Stroke, Ui, UiBuilder, Vec2, Widget};

#[derive(Clone)]
pub struct SpreadSheetWidget {
    dimension: (usize, usize),
    show_area: Option<Rect>,
    id_salt: Option<Id>,
}

#[derive(Clone)]
pub struct SpreadsheetMetadata {
    pub cursor: Option<(usize, usize)>,
    pub column_widths: SpreadsheetWidths,
    pub row_heights: SpreadsheetWidths,
}

impl SpreadsheetMetadata {
    pub fn total_internal_size(&self) -> Vec2 {
        Vec2::new(
            self.column_widths.total_width(),
            self.row_heights.total_width(),
        )
    }
}

/*
struct Spreadsheet {
    dimension: (usize, usize),
    id_salt: Option<Id>,
}

#[derive(Clone, Copy, Default)]
struct SelectionDetails {
    cursor: Option<(usize, usize)>,
    //individual: Vec<(usize, usize)>,
    //column: Vec<usize>,
    //row: Vec<usize>,
    //area: Vec<((usize, usize), (usize, usize))>,
}
*/

impl SpreadSheetWidget {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            dimension: (cols, rows),
            show_area: None,
            id_salt: None,
        }
    }

    pub fn show_area(mut self, area: Rect) -> Self {
        self.show_area = Some(area);
        self
    }

    pub fn show(
        &mut self,
        ui: &mut Ui,
        meta: &mut SpreadsheetMetadata,
        mut cell_func: impl FnMut(&mut Ui, (usize, usize)),
    ) -> egui::Response {
        // Synchronize the width numbering
        let (cols, rows) = self.dimension;
        meta.row_heights.set_len(rows);
        meta.row_heights.rebuild_accum();
        meta.column_widths.set_len(cols);
        meta.column_widths.rebuild_accum();

        // Widget setup
        let resp = ui.allocate_response(meta.total_internal_size(), Sense::click_and_drag());

        if cols * rows == 0 {
            return resp;
        }

        let view_rect = self
            .show_area
            .unwrap_or(resp.rect.translate(-resp.rect.min.to_vec2()));

        let parent_id = ui.next_auto_id();

        // Draw the background
        let line_color = Stroke::new(1., Color32::WHITE);

        let paint = ui.painter();//_at(view_rect);

        let (min_j, max_j) = meta.row_heights.range(view_rect.min.y, view_rect.max.y);
        let (min_i, max_i) = meta.column_widths.range(view_rect.min.x, view_rect.max.x);
        for j in min_j..=max_j {
            let y_offset = meta.row_heights.get_accum(j).unwrap();

            paint
                .hline(resp.rect.min.x..=resp.rect.max.x, y_offset - view_rect.min.y, line_color);
        }

        for i in min_i..=max_i {
            let x_offset = meta.column_widths.get_accum(i).unwrap();

            paint
                .vline(x_offset - view_rect.min.x, resp.rect.min.y..=resp.rect.max.y, line_color);
        }

        // Draw the contents of the cells
        for j in min_j..max_j {
            let y_offset = meta.row_heights.get_accum(j).unwrap();
            let y_height = meta.row_heights.get_width(j).unwrap();
            for i in min_i..max_i {
                let x_offset = meta.column_widths.get_accum(i).unwrap();
                let x_width = meta.column_widths.get_width(i).unwrap();

                let coord = (i, j);

                let max_rect = Rect::from_min_size(
                    resp.rect.min + Vec2::new(x_offset, y_offset),
                    Vec2::new(x_width, y_height),
                );

                let cfg = UiBuilder::new()
                    .id_salt(parent_id.with(coord))
                    .max_rect(max_rect);

                ui.allocate_new_ui(cfg, |ui| cell_func(ui, coord));
            }
        }

        resp
    }

    /// Same as using SpreadSheetWidget directly, but SpreadsheetMetadata is handled using egui.
    pub fn show_persisted_meta(
        &mut self,
        ui: &mut Ui,
        cell_ui: impl FnMut(&mut Ui, (usize, usize)),
    ) -> egui::Response {
        let id: Id = ui.next_auto_id().with(self.id_salt);
        let mut meta = ui.memory_mut(|w| {
            w.data
                .get_persisted_mut_or_default::<SpreadsheetMetadata>(id)
                .clone()
        });
        let resp = self.show(ui, &mut meta, cell_ui);
        ui.memory_mut(|w| *w.data.get_persisted_mut_or_default(id) = meta);
        resp
    }
}

#[derive(Clone)]
pub struct SpreadsheetWidths {
    default_width: f32,
    widths: Vec<f32>,
    accum: Vec<f32>,
}

impl SpreadsheetWidths {
    pub fn new(default_width: f32) -> Self {
        Self {
            default_width,
            widths: vec![],
            accum: vec![],
        }
    }

    pub fn set_len(&mut self, len: usize) {
        self.widths.resize(len, self.default_width);
    }

    pub fn get_accum(&self, idx: usize) -> Option<f32> {
        match idx.checked_sub(1) {
            Some(true_idx) => self.accum.get(true_idx).copied(),
            None => Some(0.0),
        }
    }

    pub fn get_width(&self, idx: usize) -> Option<f32> {
        self.widths.get(idx).copied()
    }

    pub fn set_width(&mut self, idx: usize, width: f32) {
        self.widths[idx] = width;
    }

    pub fn rebuild_accum(&mut self) {
        // TODO: This is stupid.
        self.accum.clear();
        self.accum.resize(self.widths.len(), 0.0);
        let mut accum = 0.0;
        for (w, a) in self.widths.iter().zip(&mut self.accum) {
            accum += w;
            *a = accum;
        }
    }

    pub fn total_width(&self) -> f32 {
        self.accum.last().copied().unwrap_or(0.0)
    }

    pub fn range(&mut self, min: f32, max: f32) -> (usize, usize) {
        (
            binary_search_sorted(&self.accum, min)
                .checked_sub(1)
                .unwrap_or(0),
            (binary_search_sorted(&self.accum, max) + 1)
                .min(self.accum.len().checked_sub(1).unwrap_or(0)),
        )
    }
}

impl Default for SpreadsheetMetadata {
    fn default() -> Self {
        Self {
            cursor: None,
            column_widths: SpreadsheetWidths::new(200.0),
            row_heights: SpreadsheetWidths::new(20.0),
        }
    }
}

fn binary_search_sorted(arr: &[f32], x: f32) -> usize {
    match arr.binary_search_by(|a| a.partial_cmp(&x).unwrap_or(std::cmp::Ordering::Equal)) {
        Err(idx) | Ok(idx) => idx,
    }
}
