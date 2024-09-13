use std::{env, process::Command};

use crate::fonts::{font_dir, fonts, Font};

pub struct State {
    pub toilet_exe: String,
    pub toilet_cmdline: String,
    pub input: String,
    pub flags: String,
    pub output: String,
    pub fonts: Vec<Font>,
    pub font_index: usize,
    pub width: usize,
}

impl State {
    // Creates a Tuilet state struct.
    pub fn init() -> Result<State, std::io::Error> {
        let toilet_exe = match env::var("TOILET") {
            Ok(exe) => exe,
            Err(_) => String::from("toilet"),
        };
        print!("{}", toilet_exe);

        let mut fonts = fonts(&font_dir(&toilet_exe)?)?;
        fonts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        Ok(State {
            toilet_exe,
            fonts,
            font_index: 0,
            input: String::from(""),
            flags: String::from(""),
            toilet_cmdline: String::from(""),
            output: String::from(""),
            width: 0,
        })
    }

    // Assembles and executes the current toilet command.
    // Sets `self.toilet_cmdline` and `self.output`.
    pub fn exec(&mut self) {
        // cmdline (self.toilet_cmdline) is intended for showing to the user.
        // real_cmdline is what we will actually run to collect output.

        let mut cmdline = self.toilet_exe.clone();
        if self.flags != "" {
            cmdline.push_str(" ");
            cmdline.push_str(&self.flags);
        }
        cmdline.push_str(" -f \"");
        cmdline.push_str(&self.font().name);
        cmdline.push_str("\" ");

        // Unless there is already a --width flag, add one to the command
        // we actually run, set to the width of our terminal
        let mut real_cmdline = cmdline.clone();
        if self.width > 0 && !&self.flags.contains("--width") {
            real_cmdline.push_str(&format!(" --width {} ", self.width - 2));
        }

        // armor quotes and backslashes in input
        let mut input = self.input.clone();
        input = input.replace("\\", "\\\\");
        input = input.replace("\"", "\\\"");
        input = format!("\"{}\"", input);

        real_cmdline.push_str(&input);
        cmdline.push_str(&input);

        self.toilet_cmdline = cmdline;

        // TODO support display of ANSI output.
        // For now, scrub those filters out of the command we actually run
        real_cmdline = real_cmdline.replace(" --gay", "");
        real_cmdline = real_cmdline.replace(" --ga", "");
        real_cmdline = real_cmdline.replace(" --g", "");
        real_cmdline = real_cmdline.replace(" --metal", "");
        real_cmdline = real_cmdline.replace(" --meta", "");
        real_cmdline = real_cmdline.replace(" --met", "");
        real_cmdline = real_cmdline.replace(" --me", "");
        real_cmdline = real_cmdline.replace(" --m", "");

        let cmd_output = Command::new("sh")
            .args(["-c", &real_cmdline])
            .output()
            .unwrap();
        self.output = String::from(String::from_utf8(cmd_output.stdout).unwrap().trim_end());
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
