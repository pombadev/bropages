mod bro;
mod cli;
mod config;

fn main() {
    let cmd = cli::new().get_matches();

    let is_search = cmd.is_present("search");
    let no_color = cmd.is_present("no-color");
    let no_paging = cmd.is_present("no-paging");

    if no_color {
        unsafe {
            config::COLOR = false;
        }
    }

    if no_paging {
        unsafe {
            config::PAGING = false;
        }
    }

    match cmd.value_of("query") {
        None => {
            cli::print_help();
        }
        Some(val) => {
            if val.trim().is_empty() {
                return println!("Query cannot be empty, please input something.\nEg: bro tar");
            }

            if is_search {
                bro::search(val);
            } else {
                bro::lookup(val);
            }
        }
    };
}
