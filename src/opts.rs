use clap::{arg, command};

pub struct Opts {
    pub font_dirs: Vec<String>,
}

impl Opts {
    pub fn from_args() -> Opts {
        let mut font_dirs: Vec<String> = Vec::new();

        let matches = command!()
            .arg(arg!(-D --fontdir <PATH> "Add fonts from the given directory").required(false))
            .get_matches();

        match matches.get_many::<String>("fontdir") {
            None => (),
            Some(dirs) => {
                dirs.for_each(|d| font_dirs.push(String::from(d)));
            }
        }

        Opts { font_dirs }
    }
}
