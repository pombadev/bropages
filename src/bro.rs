use std::{env, process, thread, time};

use bat::{PagingMode, PrettyPrinter};
use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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

pub struct Cli {
    list_themes: bool,
    no_color: bool,
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
        let mut global_settings = vec![AppSettings::ArgRequiredElseHelp];

        let show_color = if env::var_os("NO_COLOR").is_none() {
            AppSettings::ColoredHelp
        } else {
            AppSettings::ColorNever
        };

        global_settings.push(show_color);

        let themes = bat::assets::HighlightingAssets::from_binary();

        let themes = themes.themes().collect::<Vec<_>>();

        let version = format!("v{}-{}", crate_version!(), env!("GIT_HASH"));

        let app = App::new(crate_name!())
        .global_settings(&global_settings[..])
        .version(version.as_str())
        .about(crate_description!())
        .args(&[
            Arg::with_name("query")
                .help("Command to lookup"),

            Arg::with_name("list-themes")
                .long("list-themes")
                .help("Display a list of supported themes for syntax highlighting.")
                .conflicts_with_all(&["theme", "search", "query"]),

            Arg::with_name("theme")
                .long("theme")
                .short("t")
                .takes_value(true)
                .possible_values(&themes[..])
                .help("Set the theme for syntax highlighting, default is `OneHalfDark`. Use '--list-themes' to see all available themes.")
                .conflicts_with_all(&["list-themes"]),

            Arg::with_name("search")
                .short("s")
                .long("search")
                .help("Search if provided query exist in the database")
                .long_help("Search if provided query exist in the database\nThis searches for entries in the http://bropages.org database")
                .takes_value(false),

            Arg::with_name("no-color")
                .long("no-color")
                .takes_value(false)
                .help("Disable colored output"),

            Arg::with_name("no-paging")
                .long("no-paging")
                .takes_value(false)
                .help("Disable piping of the output through a pager")
        ]);

        let cmd = app.get_matches();

        Self {
            list_themes: cmd.is_present("list-themes"),
            no_color: cmd.is_present("no-color"),
            no_paging: cmd.is_present("no-paging"),
            query: String::from(cmd.value_of("query").unwrap_or_default()),
            search: cmd.is_present("search"),
            theme: String::from(cmd.value_of("theme").unwrap_or("OneHalfDark")),
            themes: themes.iter().map(|s| String::from(*s)).collect::<Vec<_>>(),
        }
    }

    fn format_to_string(list: Vec<String>) -> String {
        list.iter().fold(String::new(), |mut init, curr| {
            let splits = curr.split('\n');

            let next = splits
                .filter_map(|item| {
                    if item.is_empty() {
                        return None;
                    }

                    let inner_next;

                    if item.starts_with('#') {
                        inner_next = format!("\n{}", item);
                    } else {
                        inner_next = format!("\n{}\n", item);
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

    fn lookup(&self) {
        match Self::fetch::<BroLookupResponse>(format!("/{}.json", &self.query)) {
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

    fn search(&self) {
        match Self::fetch::<BroSearchResponse>(format!("/search/{}.json", &self.query)) {
            Ok(res) => {
                let list = res.iter().map(|item| item.cmd.clone()).collect::<Vec<_>>();

                let total = list.len();

                let snippet = format!(
                    "# Total {} matches for the term '{}':\n{}",
                    total,
                    &self.query,
                    Self::format_to_string(list)
                );

                self.print(snippet.as_bytes());
            }
            Err(err) => Self::eprint_and_exit(err),
        };
    }

    fn print(&self, snippet: &[u8]) {
        let color = !self.no_color;
        let paging = if !self.no_paging {
            PagingMode::QuitIfOneScreen
        } else {
            PagingMode::Never
        };

        let displayed = PrettyPrinter::new()
            .input_from_bytes(snippet)
            .colored_output(color)
            .line_numbers(true)
            .language("bash")
            .theme(&self.theme)
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
                .print();

            thread::sleep(time::Duration::from_secs(1));
            println!();
        }
    }
}
