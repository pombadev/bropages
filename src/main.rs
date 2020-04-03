mod bro;
mod cli;

#[tokio::main]
async fn main() {
    let args = cli::new().get_matches();

    if !args.is_present("search") && !args.is_present("query") {
        cli::print_help();
        return;
    }

    let no_color = args.is_present("no-color");
    let search = args.is_present("search");
    let lookup = args.is_present("query");

    if search || lookup {
        return match args.value_of("query") {
            None => {
                cli::print_help();
            },
            Some(val) => {
                if search {
                    bro::search(val, no_color).await;
                } else if lookup {
                    bro::lookup(val, no_color).await;
                }
            },
        };
    }
}
