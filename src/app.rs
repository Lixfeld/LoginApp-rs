pub struct LoginApp {
    username: String,
    password: String,
    confirm_password: String,
}

impl Default for LoginApp {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
        }
    }
}

impl LoginApp {
    /// Called once before the first frame.
    pub fn new(_: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for LoginApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            username,
            password,
            confirm_password,
        } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Username:");
            ui.text_edit_singleline(username);

            ui.label("Password:");
            ui.text_edit_singleline(password);

            ui.label("Confirm password:");
            ui.text_edit_singleline(confirm_password);

            let is_empty = username.is_empty() || password.is_empty();
            let is_match = password.clone() == confirm_password.clone();
            ui.add_enabled(!is_empty && is_match, egui::Button::new("Sign up"));
        });
    }
}
