#![windows_subsystem = "windows"] // I think I can remove this line in the futur I need more test when I create the installer if application got cmd window or not

// I will release .iss file for the installer later

use iced::{Element, Theme, window, Size};

struct MyApp;

// Update and view implementation for the application
impl MyApp {
    fn update(&mut self, _: ()) {
        // No return, just an update of the state if necessary
    }

    fn view(&self) -> Element<()> {
        iced::widget::container(
            iced::widget::text("")
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }
}

// Default implementation
impl Default for MyApp {
    fn default() -> Self {
        Self
    }
}

// Main function for the application
fn main() -> iced::Result {
    // If you want to display something from the environment (like name - version)
    let version: &str = env!("CARGO_PKG_VERSION");
    let name: &str = env!("CARGO_PKG_NAME");
    let title_string = format!("{} - v{}", name, version);
    let title: &'static str = Box::leak(title_string.into_boxed_str());

    // Load the icon (.ico is better)
    let icon_bytes = include_bytes!("../assets/Logo_SupV.ico");
    let image = image::load_from_memory(icon_bytes)
        .expect("Impossible to load the image")
        .into_rgba8();
    let (width, height) = image.dimensions();
    
    let icon = window::icon::from_rgba(image.into_raw(), width, height)
        .expect("Impossible to create the icon");

    let mut settings = window::Settings::default();
    settings.size = Size::new(1024.0, 768.0);
    settings.icon = Some(icon);
    settings.resizable = true;
    settings.decorations = true;
    settings.position = window::Position::Centered;

    // Create the application
    iced::application(
        title,
        MyApp::update,
        MyApp::view
    )
        .theme(|_| Theme::Dark)
        .antialiasing(true)
        .window(settings)
        .run()
}