use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};

pub(crate) fn new() -> App<'static, 'static> {
    let mut global_settings = vec![AppSettings::ArgRequiredElseHelp];

    let show_color = if std::env::var_os("NO_COLOR").is_none() {
        AppSettings::ColoredHelp
    } else {
        AppSettings::ColorNever
    };

    global_settings.push(show_color);

    App::new(crate_name!())
        .global_settings(&global_settings[..])
        .version(crate_version!())
        .about(crate_description!())
        .args(&[
            Arg::with_name("query")
                .help("Command to lookup"),
            Arg::with_name("list-themes")
                .long("list-themes")
                .help("Display a list of supported themes for syntax highlighting."),
            Arg::with_name("theme")
                .long("theme")
                .short("t")
                .takes_value(true)
                .default_value("OneHalfDark")
                .possible_values(&[
                    "1337",
                    "Coldark-Cold",
                    "Coldark-Dark",
                    "DarkNeon",
                    "Dracula",
                    "GitHub",
                    "Monokai Extended",
                    "Monokai Extended Bright",
                    "Monokai Extended Light",
                    "Monokai Extended Origin",
                    "Nord",
                    "OneHalfDark",
                    "OneHalfLight",
                    "Solarized (dark)",
                    "Solarized (light)",
                    "Sublime Snazzy",
                    "TwoDark",
                    "ansi-dark",
                    "ansi-light",
                    "base16",
                    "base16-256",
                    "gruvbox",
                    "gruvbox-light",
                    "gruvbox-white",
                    "zenburn",
                ])
                .help("Set the theme for syntax highlighting. Use '--list-themes' to see all available themes."),

            Arg::with_name("search")
                .short("s")
                .long("search")
                .help("Search if provided query exist in the database")
                .long_help("Search if provided query exist in the database\nThis searches for entries in the http://bropages.org database"),

            Arg::with_name("no-color")
                .long("no-color")
                .takes_value(false)
                .help("Disable colored output"),

            Arg::with_name("no-paging")
                .long("no-paging")
                .takes_value(false)
                .help("Disable piping of the output through a pager")
        ])
}

pub(crate) fn print_help() {
    let _ = new().print_help();
    println!();
}
