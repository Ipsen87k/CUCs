use chrono::{Datelike, Local, Timelike};

pub fn get_timestamp()->String{
    format!("{}{:02}{:02}{:02}{:02}{:02}",
        Local::now().year(),Local::now().month(),Local::now().day(),Local::now().hour(),Local::now().minute(),Local::now().second())
}