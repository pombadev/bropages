use std::{env, error::Error, process};

use bat::{PagingMode, PrettyPrinter};
use clap::{Arg, ArgAction, Command};
use ureq::serde::{de::DeserializeOwned, Deserialize, Serialize};

const VERSION_STRING: &str = include_str!(concat!(env!("OUT_DIR"), "/version"));

// snippets taken from: https://google.github.io/styleguide/shellguide.html
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

#[derive(Serialize, Deserialize)]
#[serde(crate = "ureq::serde")]
struct BroLookupResponse {
    cmd: String,
    msg: String,
    up: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "ureq::serde")]
struct BroSearchResponse {
    cmd: String,
}

pub struct Cli {
    list_themes: bool,
    no_paging: bool,
    query: String,
    search: bool,
    theme: String,
    themes: Vec<String>,
}

impl Cli {
    pub fn run() {
        let app = Self::new();

        if app.list_themes {
            return app.list_themes();
        }

        if app.search {
            app.search();
        } else {
            app.lookup();
        }
    }

    fn new() -> Self {
        let themes = bat::assets::HighlightingAssets::from_binary();

        let themes = themes.themes().collect::<Vec<_>>();

        let cmd = Command::new(env!("CARGO_PKG_NAME"))
        .version(VERSION_STRING)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg_required_else_help(true)
        .args(&[
            Arg::new("query")
                .required(true)
                .action(ArgAction::Set)
                .help("Command to lookup"),

            Arg::new("list-themes")
                .long("list-themes")
                .action(ArgAction::SetTrue)
                .help("Display a list of supported themes for syntax highlighting.")
                .conflicts_with_all(["theme", "search", "query", "no-paging"]),

            Arg::new("theme")
                .long("theme")
                .short('t')
                .action(ArgAction::Set)
                // .value_parser(&themes[..])
                .help("Set the theme for syntax highlighting, default is `OneHalfDark`. Use '--list-themes' to see all available themes.")
                .conflicts_with_all(["list-themes"]),

            Arg::new("search")
                .short('s')
                .long("search")
                .action(ArgAction::SetTrue)
                .help("Search if provided query exist in the database")
                .long_help("Search if provided query exist in the database\nThis searches for entries in the http://bropages.org database"),

            Arg::new("no-paging")
                .long("no-paging")
                .help("Disable piping of the output through a pager")
        ]).get_matches();

        let theme = match cmd.get_one::<String>("theme") {
            Some(theme) => theme.clone(),
            None => "OneHalfDark".into(),
        };

        let query = match cmd.get_one::<String>("query") {
            Some(query) => query.clone(),
            None => String::new(),
        };

        let list_themes = cmd.get_one::<bool>("list-themes").unwrap_or(&false);
        let no_paging = cmd.get_one::<bool>("no-paging").unwrap_or(&false);
        let search = cmd.get_one::<bool>("search").unwrap_or(&false);

        Self {
            theme,
            query,
            list_themes: *list_themes,
            no_paging: *no_paging,
            search: *search,
            themes: themes.iter().copied().map(String::from).collect::<Vec<_>>(),
        }
    }

    fn format_to_string(list: &[String]) -> String {
        list.iter().fold(String::new(), |mut init, current| {
            let splits = current.split('\n');

            let next = splits
                .filter_map(|item| {
                    if item.is_empty() {
                        return None;
                    }

                    let inner_next = if item.starts_with('#') {
                        format!("\n{}", item)
                    } else {
                        format!("\n{}\n", item)
                    };

                    Some(inner_next)
                })
                .collect::<String>();

            init.push_str(next.as_str());

            init
        })
    }

    fn eprint_and_exit(msg: &str) {
        eprintln!("Unable to find because of:\n  - {}", msg);
        process::exit(1);
    }

    fn fetch<T: DeserializeOwned>(path: &str) -> Result<Vec<T>, Box<dyn Error>> {
        let host = option_env!("BROPAGES_BASE_URL").unwrap_or_else(|| "http://bropages.org");

        let url = format!("{}{}", host, path);

        ureq::get(&url)
            .call()?
            .into_json::<Vec<T>>()
            .map_err(Into::into)
    }

    fn lookup(&self) {
        match Self::fetch::<BroLookupResponse>(&format!("/{}.json", &self.query)) {
            Ok(mut response) => {
                response.sort_by(|a, b| a.up.cmp(&b.up));

                let list = response
                    .iter()
                    .map(|item| item.msg.clone())
                    .collect::<Vec<_>>();

                let snippet = Self::format_to_string(&list);

                self.print(snippet.as_bytes());
            }
            Err(err) => Self::eprint_and_exit(&err.to_string()),
        };
    }

    fn search(&self) {
        match Self::fetch::<BroSearchResponse>(&format!("/search/{}.json", &self.query)) {
            Ok(res) => {
                let list = res.iter().map(|item| item.cmd.clone()).collect::<Vec<_>>();

                let total = list.len();

                let snippet = format!(
                    "# Total {} matches for the term '{}':\n{}",
                    total,
                    &self.query,
                    Self::format_to_string(&list)
                );

                self.print(snippet.as_bytes());
            }
            Err(err) => Self::eprint_and_exit(&err.to_string()),
        };
    }

    fn print(&self, snippet: &[u8]) {
        let paging = if self.no_paging {
            PagingMode::Never
        } else {
            PagingMode::QuitIfOneScreen
        };

        let displayed = PrettyPrinter::new()
            .input_from_bytes(snippet)
            .line_numbers(true)
            .language("bash")
            .theme(&self.theme)
            .colored_output(env::var_os("NO_COLOR").is_none())
            .paging_mode(paging)
            .print()
            .unwrap_or(false);

        if !displayed {
            eprintln!("Warning: syntax highlight failed.");
            println!("{}", String::from_utf8_lossy(snippet));
        }
    }

    fn list_themes(&self) {
        let mut printer = PrettyPrinter::new();

        for theme in &self.themes {
            println!("Theme: {}", theme);
            let _ = printer
                .input_from_bytes(BASH_SYNTAX_DEMO.as_bytes())
                .colored_output(true)
                .language("bash")
                .theme(theme)
                .grid(true)
                .line_numbers(true)
                .print();

            println!();
        }
    }
}
