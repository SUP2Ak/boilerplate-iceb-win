#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/Logo_SupV.ico"); // Chemin vers votre icône
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}