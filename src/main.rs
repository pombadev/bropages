mod bro;
mod cli;

pub(crate) static mut COLOR: bool = true;
pub(crate) static mut PAGING: bool = true;

fn main() {
    let cmd = cli::new().get_matches();

    if !cmd.is_present("search") && !cmd.is_present("query") {
        cli::print_help();
        return;
    }

    let no_color = cmd.is_present("no-color");
    let search = cmd.is_present("search");
    let lookup = cmd.is_present("query");
    let no_paging = cmd.is_present("no-paging");

    if no_color {
        unsafe {
            COLOR = false;
        }
    }

    if no_paging {
        unsafe {
            PAGING = false;
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
