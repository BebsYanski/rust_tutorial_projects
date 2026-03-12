use flate2::{Compression, write::GzEncoder};
use iced::event::Event;
use iced::subscription;
use iced::widget::{column, container, text};
use iced::{Application, Command, Element, Settings, Subscription, Theme, executor};
use std::fs::{File, Metadata};
use std::io::{Read, Write};

fn main() -> iced::Result {}
