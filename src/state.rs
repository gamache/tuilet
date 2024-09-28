use ansi_to_tui::IntoText;
use std::process::Command;

use crate::{
    fonts::{default_font_dir, get_fonts_from_dir, Font},
    opts::Opts,
};

#[derive(Debug)]
pub struct State<'a> {
    // Path to toilet executable
    pub toilet_exe: String,

    // The command line that the user asked for
    pub toilet_cmdline: String,

    // The output of toilet_cmdline, suitable for copying but not
    // for displaying inside Tuilet
    pub toilet_cmdline_output: String,

    // The output of a command line very much like toilet_cmdline, whose
    // output is suitable for displaying inside Tuilet but not copying
    pub output: ratatui::text::Text<'a>,

    // Text to render
    pub input: String,

    // Flags to pass to toilet
    pub flags: String,

    // Width of current window
    pub width: usize,

    // toilet_exe's built in font directory
    pub default_font_dir: String,

    // All known fonts
    pub fonts: Vec<Font>,

    // Index of currently selected font
    pub font_index: usize,
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
            toilet_cmdline_output: String::from(""),
            output: "".into_text().unwrap(),
            width: 0,
            default_font_dir,
        }
    }

    // Assembles and executes the current toilet command.
    // Sets `self.toilet_cmdline` and `self.output`.
    pub fn exec(&mut self) {
        // toilet_cmdline (self.toilet_cmdline) is the cmdline the user asked for.
        // internal_cmdline is what we will actually run to collect output.

        let mut toilet_cmdline = self.toilet_exe.clone();
        if !self.flags.is_empty() {
            toilet_cmdline.push(' ');
            toilet_cmdline.push_str(&self.flags);
        }
        toilet_cmdline.push_str(" -f \"");
        toilet_cmdline.push_str(&self.font().name);
        toilet_cmdline.push('"');
        if self.font().dir != self.default_font_dir {
            toilet_cmdline.push_str(" -d \"");
            toilet_cmdline.push_str(&self.font().dir);
            toilet_cmdline.push('"');
        }

        // Unless there is already a --width flag, add one to the command
        // we actually run, set to the width of our terminal
        let mut internal_cmdline = toilet_cmdline.clone();
        if self.width > 0 && !&self.flags.contains("--width") {
            internal_cmdline.push_str(&format!(" --width {}", self.width - 2));
        }

        // armor quotes and backslashes in input
        let mut input = self.input.clone();
        input = input.replace('\\', "\\\\");
        input = input.replace('"', "\\\"");

        input = format!(" \"{}\"", input);
        internal_cmdline.push_str(&input);
        toilet_cmdline.push_str(&input);

        self.toilet_cmdline = toilet_cmdline.clone();

        let internal_cmdline_output = Command::new("sh")
            .args(["-c", &internal_cmdline])
            .output()
            .unwrap();
        self.output = String::from_utf8(internal_cmdline_output.stdout)
            .unwrap()
            .trim_end()
            .into_text()
            .unwrap();

        let toilet_cmdline_output = Command::new("sh")
            .args(["-c", &toilet_cmdline])
            .output()
            .unwrap();
        self.toilet_cmdline_output = String::from_utf8(toilet_cmdline_output.stdout).unwrap();
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
