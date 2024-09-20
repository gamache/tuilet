use clap::{arg, command, ArgAction};

#[derive(Debug)]
pub struct Opts {
    pub font_dirs: Vec<String>,
    pub toilet_exe: String,
}

impl Opts {
    pub fn from_args(args: impl Iterator<Item = String>) -> Opts {
        let mut font_dirs: Vec<String> = Vec::new();

        let matches = command!()
            .arg(
                arg!(-D --fontdir <PATH> "Add fonts from the given directory")
                    .required(false)
                    .action(ArgAction::Append),
            )
            .arg(arg!(-X --toilet <PATH> "Set path to `toilet` executable").required(false))
            .get_matches_from(args);

        match matches.get_many::<String>("fontdir") {
            None => (),
            Some(dirs) => {
                dirs.for_each(|d| font_dirs.push(String::from(d)));
            }
        }

        let toilet_exe = match matches.get_one::<String>("toilet") {
            None => String::from("toilet"),
            Some(path) => String::from(path),
        };

        Opts {
            font_dirs,
            toilet_exe,
        }
    }
}
