use std::path::PathBuf;

use sys_locale::get_locale;

use crate::util::game::init_game_path;

pub fn init_launcher() {
    init_game_path(PathBuf::from(".minecraft")).expect("Failed to initialize game path");
}

pub fn get_launcher_language_tag() -> String {
    if let Some(system_language) = get_locale() {
        let lang = system_language.to_lowercase();
        if lang.contains("zh")
            && (lang.contains("cn") || lang.contains("hans") || lang.contains("sg"))
        {
            return "zh-hans".to_string();
        } else if lang.contains("zh")
            && (lang.contains("tw") || lang.contains("hk") || lang.contains("hant"))
        {
            return "zh-hant".to_string();
        } else if lang.contains("ja") {
            return "ja".to_string();
        } else if lang.contains("en") {
            return "en".to_string();
        } else {
            return "en".to_string();
        }
    } else {
        return "en".to_string();
    }
}
