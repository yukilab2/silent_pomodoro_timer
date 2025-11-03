use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlashColor {
    White,
    Black,
    Red,
    Blue,
    Green,
}

impl FlashColor {
    pub fn to_rgb(&self) -> [u8; 3] {
        match self {
            FlashColor::White => [255, 255, 255],
            FlashColor::Black => [0, 0, 0],
            FlashColor::Red => [255, 0, 0],
            FlashColor::Blue => [0, 0, 255],
            FlashColor::Green => [0, 255, 0],
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "white" => FlashColor::White,
            "black" => FlashColor::Black,
            "red" => FlashColor::Red,
            "blue" => FlashColor::Blue,
            "green" => FlashColor::Green,
            _ => FlashColor::White,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FlashColor::White => "white".to_string(),
            FlashColor::Black => "black".to_string(),
            FlashColor::Red => "red".to_string(),
            FlashColor::Blue => "blue".to_string(),
            FlashColor::Green => "green".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub flash_color: FlashColor,
    pub work_duration_seconds: u64,
    pub break_duration_seconds: u64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            flash_color: FlashColor::White,
            work_duration_seconds: 25 * 60,
            break_duration_seconds: 5 * 60,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        if let Some(path) = Self::settings_path() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(settings) = serde_ini::from_str::<SettingsIni>(&content) {
                    return Settings {
                        flash_color: FlashColor::from_string(&settings.flash_color),
                        work_duration_seconds: settings.work_duration_seconds,
                        break_duration_seconds: settings.break_duration_seconds,
                    };
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::settings_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let ini = SettingsIni {
                flash_color: self.flash_color.to_string(),
                work_duration_seconds: self.work_duration_seconds,
                break_duration_seconds: self.break_duration_seconds,
            };
            if let Ok(content) = serde_ini::to_string(&ini) {
                let _ = std::fs::write(&path, content);
            }
        }
    }

    fn settings_path() -> Option<PathBuf> {
        ProjectDirs::from("", "", "pomodoro")
            .map(|dirs| dirs.config_dir().join("settings.ini"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SettingsIni {
    flash_color: String,
    work_duration_seconds: u64,
    break_duration_seconds: u64,
}

