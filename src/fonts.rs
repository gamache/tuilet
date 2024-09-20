use regex::Regex;
use std::{fs, process::Command};

#[derive(Debug)]
pub struct Font {
    pub name: String,
    pub dir: String,
}

pub fn default_font_dir(toilet_exe: &str) -> String {
    let cmd_output = Command::new(toilet_exe).args(["-I", "2"]).output().unwrap();

    String::from(String::from_utf8(cmd_output.stdout).unwrap().trim_end())
}

pub fn get_fonts_from_dir(dir: &str) -> Vec<Font> {
    match maybe_fonts(dir) {
        Ok(fts) => fts,
        Err(_) => {
            let empty_fonts: Vec<Font> = Vec::new();
            empty_fonts
        }
    }
}

fn maybe_fonts(dir: &str) -> Result<Vec<Font>, std::io::Error> {
    let mut fonts: Vec<Font> = Vec::new();
    for file in fs::read_dir(dir)? {
        let path_buf = file?.path();
        let path = path_buf.to_str().unwrap();
        if path.ends_with(".tlf") || path.ends_with(".flf") {
            let relative_re = Regex::new(r".+/").unwrap();
            let relative = relative_re.replace(path, ""); //path.trim_start_matches(&format!("{}/", dir));``
            let len = relative.len() - 4;
            let name = &relative[..len];

            fonts.push(Font {
                name: String::from(name),
                dir: String::from(dir),
            });
        }
    }
    Ok(fonts)
}
