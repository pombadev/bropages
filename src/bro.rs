use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BroLookupResponse {
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

pub fn convert_list_to_string(list: Vec<&String>) -> String {
    list.iter().fold(String::new(), |mut acc, item| {
        acc.push_str(format!("{}\n", item).as_str());
        acc
    })
}

fn eprint_and_exit(msg: String) {
    eprintln!("{}: {}", "error".bright_red().bold(), msg);
    std::process::exit(1);
}

pub async fn lookup(query: &str, no_color: bool) {
    let url = format!("http://bropages.org/{}.json", query);

    match reqwest::get(&url).await {
        Ok(response) => {
            let status = response.status();

            if status.as_str() == "200" {
                match response.json::<Vec<BroLookupResponse>>().await {
                    Ok(res) => {
                        let list = res.iter().map(|item| &item.msg).collect::<Vec<&String>>();

                        let snippet = convert_list_to_string(list);

                        if no_color {
                            println!("{}", snippet);
                        } else {
                            write_to_stdio(snippet.as_str());
                        }
                    }
                    Err(err) => eprint_and_exit(err.to_string()),
                }
            } else {
                eprint_and_exit(format!("{}", status));
            }
        }
        // usually network error
        Err(err) => eprint_and_exit(err.to_string()),
    };
}

pub async fn search(query: &str, no_color: bool) {
    let url = format!("http://bropages.org/search/{}.json", query);

    match reqwest::get(&url).await {
        Ok(response) => {
            let status = response.status();

            if status.as_str() == "200" {
                match response.json::<Vec<BroSearchResponse>>().await {
                    Ok(res) => {
                        let list = res.iter().map(|item| &item.cmd).collect::<Vec<&String>>();

                        let total = list.len();

                        let snippet = format!(
                            "# There {} total '{}' results for the term '{}':\n\n{}",
                            if total > 1 { "are" } else { "is" },
                            total,
                            query,
                            convert_list_to_string(list)
                        );

                        if no_color {
                            println!("{}", snippet);
                        } else {
                            write_to_stdio(snippet.as_str());
                        }
                    }
                    Err(err) => eprint_and_exit(err.to_string()),
                }
            } else {
                eprint_and_exit(format!("{}", status));
            }
        }
        // usually network error
        Err(err) => eprint_and_exit(err.to_string()),
    };
}

pub fn write_to_stdio(snippet: &str) {
    let mut final_string_to_print = String::new();
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

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();

    let syntax = syntax_set
        .find_syntax_by_extension("bash")
        .expect("Unable to find syntax definition for `bash`");
    let mut highlighter = HighlightLines::new(syntax, &theme_set.themes["base16-mocha.dark"]);

    for line in LinesWithEndings::from(snippet) {
        // LinesWithEndings enables use of newlines mode
        let ranges: Vec<(Style, &str)> = highlighter.highlight(line, &syntax_set);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        final_string_to_print.push_str(escaped.as_str())
    }

    println!("{}", final_string_to_print.as_str());
}
