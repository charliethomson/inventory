use crate::item::Item;
use directories::ProjectDirs;
use iced::{
    slider, Align, Color, Column, Element, HorizontalAlignment, Length, Point, Row, Sandbox,
    Settings, Size, Slider, Text, Vector, VerticalAlignment,
};
use std::path::PathBuf;

fn default_path() -> PathBuf {
    ProjectDirs::from("com", "charliecthomson", "portalinvapp")
        .expect("Failed to get default backup path")
        .data_dir()
        .to_path_buf()
}

#[derive(Clone, Debug)]
pub enum Message {
    BarcodeScanned(String),
}

pub struct UI {
    scans: Vec<Item>,
    path: PathBuf,
}
impl Sandbox for UI {
    type Message = Message;

    fn new() -> Self {
        println!("{:?}", default_path());
        Self {
            scans: Vec::new(),
            path: default_path(),
        }
    }

    fn title(&self) -> String {
        String::from("UBIF INTERNAL - Inventory manager")
    }

    fn update(&mut self, _message: Self::Message) {
        // This application has no interactions
    }

    fn view(&mut self) -> Element<Self::Message> {
        Text::new("Hello, world!").into()
    }
}
