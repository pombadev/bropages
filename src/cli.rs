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

        Arg::with_name("search")
            .short("s")
            .long("search")
            .help("Search if provided query exist in the database")
            .long_help("Search if provided query exist in the database\nThis searches for entries in the http://bropages.org database"),

            Arg::with_name("no-color")
                .long("no-color")
                .help("Disable colored output"),

        Arg::with_name("no-paging")
            .long("no-paging")
            .help("Disable piping of the output through a pager")
    ])
}

pub(crate) fn print_help() {
    let _ = new().print_help();
    println!();
}
