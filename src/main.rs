mod bro;
mod cli;
mod config;

fn main() {
    let cmd = cli::new().get_matches();

    let no_color = cmd.is_present("no-color");
    let search = cmd.is_present("search");
    let lookup = cmd.is_present("query");
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

    if search || lookup {
        return match cmd.value_of("query") {
            None => {
                cli::print_help();
            }
            Some(val) => {
                if search {
                    bro::search(val);
                } else if lookup {
                    bro::lookup(val);
                }
            }
        };
    }
}
