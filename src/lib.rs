use egui::{Id, Pos2, Rect, Sense, Ui, Vec2, Widget};

#[derive(Default, Clone)]
pub struct SpreadSheetWidget {
    dimension: (usize, usize),
    //id_salt: Option<Id>,
}

#[derive(Default, Clone)]
pub struct SpreadsheetMetadata {
    pub cursor: Option<(usize, usize)>,
    pub column_widths: SpreadsheetWidths,
    pub row_heights: SpreadsheetWidths,
}

impl SpreadsheetMetadata {
    pub fn total_internal_size(&self) -> Vec2 {
        Vec2::new(self.column_widths.total_width(), self.row_heights.total_width())
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
    fn show(
        self,
        ui: &mut Ui,
        meta: &mut SpreadsheetMetadata,
        view_sub_area: Rect,
        cell_ui: impl FnMut(&mut Ui, (usize, usize)) -> egui::Response,
    ) -> egui::Response {
        let resp = ui.allocate_response(meta.total_internal_size(), Sense::click_and_drag());

        if resp.clicked() {
        }

        resp
    }
}

/*

impl Default for Spreadsheet {
    fn default() -> Self {
        Self {
            dimension: (3, 3),
            size: Vec2::splat(200.),
            id_salt: None,
        }
    }
}
*/

#[derive(Default, Clone)]
struct SpreadsheetWidths {
    widths: Vec<f32>,
    accum: Vec<f32>,
}

impl SpreadsheetWidths {
    pub fn set_width(&mut self, idx: usize, width: f32) {
        self.widths[idx] = width;
        self.rebuild_accum();
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
}
