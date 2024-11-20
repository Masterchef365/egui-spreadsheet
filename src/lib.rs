use egui::{Id, Pos2, Rect, ScrollArea, Sense, Ui, Vec2, Widget};

#[derive(Clone)]
pub struct SpreadSheetWidget {
    dimension: (usize, usize),
    show_area: Option<Rect>,
    id_salt: Option<Id>,
}

#[derive(Default, Clone)]
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
        cell_ui: impl FnMut(&mut Ui, (usize, usize)),
    ) -> egui::Response {
        // Synchronize the width numbering
        let (cols, rows) = self.dimension;
        meta.row_heights.set_len(rows);
        meta.column_widths.set_len(cols);

        let resp = ui.allocate_response(meta.total_internal_size(), Sense::click_and_drag());

        if resp.clicked() {}

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

#[derive(Default, Clone)]
pub struct SpreadsheetWidths {
    default_width: f32,
    widths: Vec<f32>,
    accum: Vec<f32>,
}

impl SpreadsheetWidths {
    pub fn set_len(&mut self, len: usize) {
        self.widths.resize(len, self.default_width);
    }

    pub fn get_width(&self, idx: usize) -> Option<f32> {
        self.widths.get(idx).copied()
    }

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
