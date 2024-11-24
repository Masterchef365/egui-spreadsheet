use std::{borrow::Cow, collections::HashMap};

use egui::{CentralPanel, Layout, ScrollArea, TextEdit, Ui, UiBuilder};
use egui_spreadsheet::SpreadSheetWidget;

fn main() {
    let mut cells: HashMap<(usize, usize), String> = HashMap::new();

    eframe::run_simple_native("Demo Sheet", Default::default(), move |ctx, _frame| {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("This is a spreadsheet widget:");
            ScrollArea::both().show_viewport(ui, |ui, viewport| {
                SpreadSheetWidget::new(100, 100)
                    .show_area(viewport)
                    .show_persisted_meta(ui, |ui, coord, rect| {
                        let cell = cells.get(&coord);
                        let mut s = cell.cloned().unwrap_or_else(String::new);

                        let resp = ui.add(TextEdit::singleline(&mut s).desired_width(200.0 - 30.0));

                        if resp.changed() {
                            cells.insert(coord, s);
                        }
                    });
            });
        });
    })
    .unwrap();
}
