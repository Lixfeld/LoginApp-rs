use regex::Regex;

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

            let regex = Regex::new(r"^[A-Za-z0-9]*$").expect("Valid regex pattern");
            let valid_username = username.len() >= 3 && regex.is_match(username);
            let username_error = if username.len() < 3 {
                "Username must be at least 3 chars."
            } else {
                "Username contains invalid characters."
            };
            ui.add_visible(!valid_username, egui::Label::new(username_error));

            ui.label("Password:");
            ui.text_edit_singleline(password);

            let valid_password = password.len() >= 6;
            ui.add_visible(
                !valid_password,
                egui::Label::new("Password must be at least 6 chars."),
            );

            ui.label("Confirm password:");
            ui.text_edit_singleline(confirm_password);

            let is_match = password.clone() == confirm_password.clone();
            ui.add_visible(!is_match, egui::Label::new("Passwords must match."));

            let is_valid = valid_username && valid_password && is_match;
            ui.add_enabled(is_valid, egui::Button::new("Sign up"));
        });
    }
}
