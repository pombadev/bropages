mod bro;

#[tokio::main]
async fn main() {
    let args = bro::cli().get_matches();

    if args.is_present("search") {
        bro::search(args.value_of("query").unwrap()).await;
        return;
    }

    if args.is_present("query") {
        bro::lookup(args.value_of("query").unwrap()).await;
        return;
    }
}
