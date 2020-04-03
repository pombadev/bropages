use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};

pub fn new() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .settings(&[
            AppSettings::ColorAuto,
            AppSettings::ColoredHelp,
            AppSettings::UnifiedHelpMessage,
        ])
    .args(&[
        Arg::with_name("query")
            .help("Command to lookup"),

        Arg::with_name("search")
            .short("l")
            .long("lookup")
            .help("Lookup an entry, bro, or just call bro")
            .long_help("Lookup an entry, bro, or just call bro\nThis looks up entries in the http://bropages.org database."),

        Arg::with_name("no-color")
            .long("no-color")
            .help("Disable syntax highlighting"),

        // Arg::with_name("pager")
        //     .short("p")
        //     .long("pager")
        //     .help("Control piping of the output through a pager")
    ])
}

pub fn print_help() {
    new().print_help().unwrap();
    println!("\n");
}
