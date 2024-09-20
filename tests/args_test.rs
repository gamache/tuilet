use tuilet::opts::Opts;

#[test]
fn it_parses_right() {
    let args = [
        String::from("tuilet"),
        String::from("-D"),
        String::from("dir1"),
        String::from("-D"),
        String::from("dir2"),
        String::from("-X"),
        String::from("exe"),
    ]
    .into_iter();

    let opts = Opts::from_args(args);

    assert_eq!(
        opts.font_dirs,
        vec![String::from("dir1"), String::from("dir2")]
    );

    assert_eq!(opts.toilet_exe, String::from("exe"));
}
