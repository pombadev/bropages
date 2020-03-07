use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BroResponse {
    cmd: String,
    msg: String,
    updated_at: String,
    id: i32,
    up: i32,
    down: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroSearchResponse {
    cmd: String,
    id: i32,
}

pub fn get_snippet_from_list(list: Vec<String>) -> String {
    list.iter().fold(String::new(), |mut acc, item| {
        acc.push_str(format!("{}\n", item).as_str());
        acc
    })
}

pub async fn lookup(query: &str) {
    let url = format!("http://bropages.org/{}.json", query);
    let response = reqwest::get(&url).await;

    // TODO: abstract away error handling, prevent duplicate
    if let Err(error) = response {
        eprintln!("{}", error.to_string())
    } else if let Ok(response) = response {
        let response = response.json::<Vec<BroResponse>>().await;

        if let Err(error) = response {
            // println!("{:#?}", error);
            eprintln!("{}", error.to_string());
        } else if let Ok(mut response) = response {
            response.sort_by(|first, second| second.up.cmp(&first.up));

            let list = response
                .iter()
                .map(|item| item.msg.clone())
                .collect::<Vec<String>>();

            let snippet = get_snippet_from_list(list);

            paint(snippet.as_str());
        }
    }
}

pub async fn search(query: &str) {
    let url = format!("http://bropages.org/search/{}.json", query);
    let response = reqwest::get(&url).await;

    // TODO: abstract away error handling, prevent duplicate
    if let Err(error) = response {
        eprintln!("{}", error.to_string())
    } else if let Ok(response) = response {
        let response = response.json::<Vec<BroSearchResponse>>().await;

        if let Err(error) = response {
            // println!("{:#?}", error);
            eprintln!("{}", error.to_string());
        } else if let Ok(response) = response {
            let list = response
                .iter()
                .map(|item| item.cmd.clone())
                .collect::<Vec<String>>();

            let snippet = get_snippet_from_list(list);

            paint(snippet.as_str());
        }
    }
}

pub fn cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::ColorAuto,
        ])
    .args(&[
        Arg::with_name("search")
            .short("l")
            .long("lookup")
            .help("Lookup an entry, bro, or just call bro")
            .long_help("Lookup an entry, bro, or just call bro\nThis looks up entries in the http://bropages.org database."),
        Arg::from_usage("<query> 'Command to lookup'")
    ])
}

pub fn paint(snippet: &str) {
    use syntect::{
        easy::HighlightLines,
        highlighting::{Style, ThemeSet},
        parsing::SyntaxSet,
        util::{as_24_bit_terminal_escaped, LinesWithEndings},
    };
    // Available themes:
    // base16-ocean.dark
    // base16-eighties.dark
    // base16-mocha.dark
    // base16-ocean.light
    // InspiredGitHub
    // Solarized (dark)
    // Solarized (light)

    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("bash").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-mocha.dark"]);

    for line in LinesWithEndings::from(snippet) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        print!("{}", escaped);
    }
}
