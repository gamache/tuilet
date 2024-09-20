use ansi_to_tui::IntoText;
use std::process::Command;

use crate::{
    fonts::{default_font_dir, get_fonts_from_dir, Font},
    opts::Opts,
};

#[derive(Debug)]
pub struct State<'a> {
    pub toilet_exe: String,
    pub toilet_cmdline: String,
    pub input: String,
    pub flags: String,
    pub output: ratatui::text::Text<'a>,
    pub fonts: Vec<Font>,
    pub font_index: usize,
    pub width: usize,
    pub default_font_dir: String,
}

// Returns true if `toilet_exe` behaves like toilet.
fn verify_toilet_exe(toilet_exe: &String) -> bool {
    let cmd_output = Command::new(toilet_exe)
        .args(["-f", "term", "hello"])
        .output();

    match cmd_output {
        Ok(output) => {
            let output_text = String::from(String::from_utf8(output.stdout).unwrap().trim_end());

            "hello" == output_text
        }
        Err(e) => {
            println!("{}", e);
            false
        }
    }
}

impl State<'_> {
    // Creates a Tuilet state struct.
    pub fn new(opts: &Opts) -> State {
        let toilet_exe = opts.toilet_exe.clone();
        if !verify_toilet_exe(&toilet_exe) {
            panic!("{} is not a working toilet", toilet_exe);
        }

        let default_font_dir = default_font_dir(&toilet_exe);

        let mut fonts: Vec<Font> = Vec::new();
        fonts.append(&mut get_fonts_from_dir(&default_font_dir));
        for dir in &opts.font_dirs {
            fonts.append(&mut get_fonts_from_dir(dir));
        }
        fonts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        State {
            toilet_exe,
            fonts,
            font_index: 0,
            input: String::from(""),
            flags: String::from(""),
            toilet_cmdline: String::from(""),
            output: "".into_text().unwrap(),
            width: 0,
            default_font_dir,
        }
    }

    // Assembles and executes the current toilet command.
    // Sets `self.toilet_cmdline` and `self.output`.
    pub fn exec(&mut self) {
        // cmdline (self.toilet_cmdline) is intended for showing to the user.
        // real_cmdline is what we will actually run to collect output.

        let mut cmdline = self.toilet_exe.clone();
        if !self.flags.is_empty() {
            cmdline.push(' ');
            cmdline.push_str(&self.flags);
        }
        cmdline.push_str(" -f \"");
        cmdline.push_str(&self.font().name);
        cmdline.push('"');
        if self.font().dir != self.default_font_dir {
            cmdline.push_str(" -d \"");
            cmdline.push_str(&self.font().dir);
            cmdline.push('"');
        }

        // Unless there is already a --width flag, add one to the command
        // we actually run, set to the width of our terminal
        let mut real_cmdline = cmdline.clone();
        if self.width > 0 && !&self.flags.contains("--width") {
            real_cmdline.push_str(&format!(" --width {}", self.width - 2));
        }

        // armor quotes and backslashes in input
        let mut input = self.input.clone();
        input = input.replace('\\', "\\\\");
        input = input.replace('"', "\\\"");

        input = format!(" \"{}\"", input);
        real_cmdline.push_str(&input);
        cmdline.push_str(&input);

        self.toilet_cmdline = cmdline;

        let cmd_output = Command::new("sh")
            .args(["-c", &real_cmdline])
            .output()
            .unwrap();
        self.output = String::from_utf8(cmd_output.stdout)
            .unwrap()
            .trim_end()
            .into_text()
            .unwrap();
    }

    // Returns the currently selected font.
    pub fn font(&self) -> &Font {
        &self.fonts[self.font_index]
    }

    // Select the next font in the list, returning it.
    pub fn next_font(&mut self) -> &Font {
        self.font_index = (self.font_index + 1) % self.fonts.len();
        self.font()
    }

    // Select the previous font in the list, returning it.
    pub fn prev_font(&mut self) -> &Font {
        let len = self.fonts.len();
        self.font_index = (self.font_index + len - 1) % len;
        self.font()
    }
}
