use egui::*;
use regex::Regex;

pub struct LoginApp {
    username: String,
    password: String,
    confirm_password: String,
    dialog_open: bool,
}

impl Default for LoginApp {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            confirm_password: String::new(),
            dialog_open: false,
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
            dialog_open,
        } = self;

        // Show Dialog
        if *dialog_open {
            let dialog_text = format!("User '{}' created successfully.", username);
            Window::new("Dialog")
                .title_bar(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(dialog_text);

                    let ok_button = ui.button("OK");
                    if ok_button.clicked() {
                        self.dialog_open = false;
                    }
                });
        }

        CentralPanel::default().show(ctx, |ui| {
            // Disable panel in background
            ui.set_enabled(!self.dialog_open);

            // Username
            ui.label("Username:");
            ui.text_edit_singleline(username);

            let regex = Regex::new(r"^[A-Za-z0-9]*$").expect("Valid regex pattern");
            let valid_username = username.len() >= 3 && regex.is_match(username);
            let username_error = if username.len() < 3 {
                "Username must be at least 3 chars."
            } else {
                "Username contains invalid characters."
            };
            let username_error_text = RichText::new(username_error).color(Color32::RED);
            ui.add_visible(!valid_username, Label::new(username_error_text));

            // Password
            ui.label("Password:");
            ui.text_edit_singleline(password);

            let valid_password = password.len() >= 6;
            let password_error_text =
                RichText::new("Password must be at least 6 chars.").color(Color32::RED);
            ui.add_visible(!valid_password, Label::new(password_error_text));

            // Confirm Password
            ui.label("Confirm password:");
            ui.text_edit_singleline(confirm_password);

            let is_match = password.clone() == confirm_password.clone();
            let confirm_password_error_text =
                RichText::new("Passwords must match.").color(Color32::RED);
            ui.add_visible(!is_match, Label::new(confirm_password_error_text));

            // Sign up
            let is_valid = valid_username && valid_password && is_match;
            let signup_button = ui.add_enabled(is_valid, Button::new("Sign up"));
            if signup_button.clicked() {
                self.dialog_open = true;
            }
        });
    }
}
