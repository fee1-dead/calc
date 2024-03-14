#[derive(Default)]
struct App {
    operation: Option<Box<dyn FnOnce(u64) -> u64>>,
    value: u64,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label(self.value.to_string());
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            for num in [1, 4, 7] {
                ui.horizontal(|ui| {
                    for num in num..=num+2 {
                        let numtext = num.to_string();
                        if ui.button(&numtext).clicked() {
                            self.value = self.value * 10 + num;
                        }
                    }
                });
            }
            if ui.button("0").clicked() {
                self.value *= 10;
            }
            if ui.button("+").clicked() {
                let num = self.value;
                self.operation = Some(Box::new(move |n| { n + num }));
                self.value = 0;
            }
            if ui.button("=").clicked() {
                if let Some(op) = self.operation.take() {
                    self.value = op(self.value);
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    eframe::run_native("calculator", native_options, Box::new(|_| Box::new(App::default())))
}
