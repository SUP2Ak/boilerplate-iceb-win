#![windows_subsystem = "windows"] 

//use std::{f32::consts::PI, time::Instant};
use iced::{Element, Theme, window, Size};

struct MyApp;

pub fn main() -> iced::Result {
    let version: &str = env!("CARGO_PKG_VERSION");
    let title_string = format!("everysup - v{}", version);
    let title: &'static str = Box::leak(title_string.into_boxed_str());

    // Charger et décoder l'icône PNG
    let icon_bytes = include_bytes!("../assets/Logo_SupV.ico");
    let image = image::load_from_memory(icon_bytes)
        .expect("Impossible de charger l'image")
        .into_rgba8();
    let (width, height) = image.dimensions();
    
    let icon = window::icon::from_rgba(image.into_raw(), width, height)
        .expect("Impossible de créer l'icône");

    let mut settings = window::Settings::default();
    settings.size = Size::new(1024.0, 768.0);
    settings.icon = Some(icon);
    settings.resizable = true;
    settings.decorations = true;
    settings.position = window::Position::Centered;

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

impl MyApp {
    fn update(&mut self, _: ()) {
        // Pas de retour, juste une mise à jour de l'état si nécessaire
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

impl Default for MyApp {
    fn default() -> Self {
        Self
    }
}
