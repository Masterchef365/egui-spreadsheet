use egui::{CentralPanel, ScrollArea};
use egui_spreadsheet::SpreadSheetWidget;

fn main() {
    eframe::run_simple_native("Demo Sheet", Default::default(), move |ctx, _frame| {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("This is a spreadsheet widget:");
            ScrollArea::both().show_viewport(ui, |ui, viewport| {
                SpreadSheetWidget::new(10, 10)
                    .show_area(viewport)
                    .show_persisted_meta(ui, |ui, (col, row)| {
                        ui.label(format!("I'm a cell at ({col}, {row})"));
                    });
            });
        });
    })
    .unwrap();
}
