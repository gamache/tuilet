use std::{fs, io::Error, process::Command};

#[derive(Debug)]
pub struct Font {
    pub name: String,
    pub dir: String,
}

pub fn fonts(dir: &str) -> Result<Vec<Font>, std::io::Error> {
    let mut fonts: Vec<Font> = Vec::new();
    for file in fs::read_dir(dir)? {
        let path_buf = file?.path();
        let path = path_buf.to_str().unwrap();
        if path.ends_with(".tlf") || path.ends_with(".flf") {
            let relative = path.trim_start_matches(&format!("{}/", dir));
            let len = relative.len() - 4;
            let name = &String::from(relative)[..len];

            fonts.push(Font {
                name: String::from(name),
                dir: String::from(dir),
            });
        }
    }
    Ok(fonts)
}

pub fn font_dir(toilet_cmd: &str) -> Result<String, Error> {
    let cmd_output = Command::new(toilet_cmd).args(["-I", "2"]).output()?;
    Ok(String::from(
        String::from_utf8(cmd_output.stdout).unwrap().trim_end(),
    ))
}
