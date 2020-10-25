use serde::{Deserialize, Serialize};
use std::{env, process};

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

                Some(String::from(inner_next))
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

fn fetch<T: serde::de::DeserializeOwned>(path: String) -> Result<Vec<T>, String> {
    let host = env::var("BROPAGES_BASE_URL").unwrap_or("http://bropages.org".to_string());
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

pub(crate) fn lookup(query: &str) {
    match fetch::<BroLookupResponse>(format!("/{}.json", query)) {
        Ok(mut response) => {
            response.sort_by(|a, b| a.up.cmp(&b.up));

            let list = response
                .iter()
                .map(|item| item.msg.clone())
                .collect::<Vec<_>>();

            let snippet = format_to_string(list);

            print(snippet.as_bytes());
        }
        Err(err) => eprint_and_exit(err),
    };
}

pub(crate) fn search(query: &str) {
    match fetch::<BroSearchResponse>(format!("/search/{}.json", query)) {
        Ok(res) => {
            let list = res.iter().map(|item| item.cmd.clone()).collect::<Vec<_>>();

            let total = list.len();

            let snippet = format!(
                "# Total {} matches for the term '{}':\n{}",
                total,
                query,
                format_to_string(list)
            );

            print(snippet.as_bytes());
        }
        Err(err) => eprint_and_exit(err),
    };
}

fn print(snippet: &[u8]) {
    let color = unsafe { crate::COLOR };
    let paging = unsafe {
        if crate::PAGING {
            bat::PagingMode::QuitIfOneScreen
        } else {
            bat::PagingMode::Never
        }
    };

    let displayed = bat::PrettyPrinter::new()
        .input_from_bytes(snippet)
        .colored_output(color)
        .line_numbers(true)
        .language("bash")
        .paging_mode(paging)
        .print()
        .unwrap_or(false);

    if !displayed {
        println!("{:?}", snippet);
    }
}
