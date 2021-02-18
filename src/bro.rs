use std::{env, process};

use bat::{assets, PagingMode, PrettyPrinter};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// snippets taken from: https://google.github.io/styleguide/shellguide.html
const BASH_SYNTAX_DEMO: &str = r#"#!/usr/bin/env sh

# All fits on one line
command1 | command2

# Long commands
command1 \
  | command2 \
  | command3 \
  | command4

# log to stderr
err() {
  echo "[$(date +'%Y-%m-%dT%H:%M:%S%z')]: $*" >&2
}

if ! do_something; then
  err "Unable to do_something"
  exit 1
fi
"#;

#[derive(Debug, Serialize, Deserialize)]
struct BroLookupResponse {
    cmd: String,
    msg: String,
    up: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct BroSearchResponse {
    cmd: String,
}

#[derive(Debug, Default)]
struct Config {
    no_color: bool,
    no_paging: bool,
    theme: String,
}

#[derive(Debug)]
enum Mode {
    ListThemes,
    Search(String),
    Query(String),
    Unknown,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Unknown
    }
}

#[derive(Debug, Default)]
pub struct App {
    config: Config,
    mode: Mode,
}

impl App {
    pub fn run() {
        let app = Self::new();

        match app.mode {
            Mode::ListThemes => {
                Self::list_themes();
            }
            Mode::Search(ref s) => {
                app.search(s);
            }
            Mode::Query(ref s) => {
                app.lookup(s);
            }
            Mode::Unknown => {
                crate::cli::print_help();
            }
        }
    }

    fn new() -> Self {
        let cmd = crate::cli::new().get_matches();
        let no_color = cmd.is_present("no-color");
        let no_paging = cmd.is_present("no-paging");
        let query = String::from(cmd.value_of("query").unwrap_or_default());
        // can unwrap because we've default value for theme
        let theme = String::from(cmd.value_of("theme").unwrap());

        let mode = if cmd.is_present("list-themes") {
            Mode::ListThemes
        } else if cmd.is_present("search") {
            Mode::Search(query)
        } else if cmd.is_present("query") {
            Mode::Query(query)
        } else {
            Mode::Unknown
        };

        Self {
            mode,
            config: Config {
                no_color,
                no_paging,
                theme,
            },
        }
    }

    fn format_to_string(list: Vec<String>) -> String {
        list.iter().fold(String::new(), |mut init, curr| {
            let splits = curr.split('\n');

            let next = splits
                .enumerate()
                .filter_map(|(index, item)| {
                    if item.is_empty() {
                        return None;
                    }

                    let inner_next;

                    // dont append and or prepend newline(s) if current item is the first line it looks jarring.
                    if index == 0 {
                        inner_next = String::from(item);
                    } else {
                        if item.starts_with('#') {
                            inner_next = format!("\n{}", item);
                        } else {
                            inner_next = format!("\n{}\n", item);
                        }
                    }

                    Some(inner_next)
                })
                .collect::<String>();

            init.push_str(next.as_str());

            init
        })
    }

    fn eprint_and_exit(msg: String) {
        eprintln!("Unable to find because of:\n  - {}", msg);
        process::exit(1);
    }

    fn fetch<T: DeserializeOwned>(path: String) -> Result<Vec<T>, String> {
        let maybe_url = env::var("BROPAGES_BASE_URL");
        let host = &maybe_url.unwrap_or_else(|_| "http://bropages.org".into());
        let url = format!("{}{}", host, path);

        match attohttpc::get(url).send() {
            Ok(response) => {
                if response.is_success() {
                    match response.json::<Vec<T>>() {
                        Ok(res) => Ok(res),
                        Err(err) => Err(err.to_string()),
                    }
                } else {
                    Err(response.status().to_string())
                }
            }
            // usually network error
            Err(err) => Err(err.to_string()),
        }
    }

    fn lookup(&self, query: &String) {
        match Self::fetch::<BroLookupResponse>(format!("/{}.json", query)) {
            Ok(mut response) => {
                response.sort_by(|a, b| a.up.cmp(&b.up));

                let list = response
                    .iter()
                    .map(|item| item.msg.clone())
                    .collect::<Vec<_>>();

                let snippet = Self::format_to_string(list);

                self.print(snippet.as_bytes());
            }
            Err(err) => Self::eprint_and_exit(err),
        };
    }

    fn search(&self, query: &String) {
        match Self::fetch::<BroSearchResponse>(format!("/search/{}.json", query)) {
            Ok(res) => {
                let list = res.iter().map(|item| item.cmd.clone()).collect::<Vec<_>>();

                let total = list.len();

                let snippet = format!(
                    "# Total {} matches for the term '{}':\n{}",
                    total,
                    query,
                    Self::format_to_string(list)
                );

                self.print(snippet.as_bytes());
            }
            Err(err) => Self::eprint_and_exit(err),
        };
    }

    fn print(&self, snippet: &[u8]) {
        let color = !self.config.no_color;
        let paging = if !self.config.no_paging {
            PagingMode::QuitIfOneScreen
        } else {
            PagingMode::Never
        };

        let displayed = PrettyPrinter::new()
            .input_from_bytes(snippet)
            .colored_output(color)
            .line_numbers(true)
            .language("bash")
            .theme(&self.config.theme)
            .paging_mode(paging)
            .print()
            .unwrap_or(false);

        if !displayed {
            eprintln!("Warning: syntax highlight failed.");
            println!("{}", String::from_utf8_lossy(snippet));
        }
    }

    fn list_themes() {
        let mut printer = PrettyPrinter::new();

        for theme in assets::HighlightingAssets::from_binary().themes() {
            println!("Theme: {}", theme);
            let _ = printer
                .input_from_bytes(BASH_SYNTAX_DEMO.as_bytes())
                .colored_output(true)
                .language("bash")
                .theme(theme)
                .grid(true)
                .print();
            println!();
        }
    }
}
