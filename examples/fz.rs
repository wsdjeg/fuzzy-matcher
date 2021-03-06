use fuzzy_matcher::clangd::ClangdMatcher;
use fuzzy_matcher::skim::{SkimMatcher, SkimMatcherV2};
use fuzzy_matcher::FuzzyMatcher;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;
use termion::style::{Invert, Reset};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // arg parsing (manually)
    let mut arg_iter = args.iter().skip(1);
    let mut pattern = "".to_string();
    let mut algorithm = Some("skim");

    while let Some(arg) = arg_iter.next() {
        if arg == "--algo" {
            algorithm = arg_iter.next().map(String::as_ref);
        } else {
            pattern = arg.to_string();
        }
    }

    if &pattern == "" {
        println!("Usage: echo <piped_input> | fz --algo [skim|skim_v1|cland] <pattern>");
        exit(1);
    }

    let matcher: Box<dyn FuzzyMatcher> = match algorithm {
        Some("skim") | Some("skim_v2") => Box::new(SkimMatcherV2::default()),
        Some("skim_v1") => Box::new(SkimMatcher::default()),
        Some("clangd") => Box::new(ClangdMatcher::default()),
        _ => panic!("Algorithm not support {:?}", algorithm),
    };

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if let Some((score, indices)) = matcher.fuzzy_indices(&line, &pattern) {
                println!("{:8}: {}", score, wrap_matches(&line, &indices));
            }
        }
    }
}

fn wrap_matches(line: &str, indices: &[usize]) -> String {
    let mut ret = String::new();
    let mut peekable = indices.iter().peekable();
    for (idx, ch) in line.chars().enumerate() {
        let next_id = **peekable.peek().unwrap_or(&&line.len());
        if next_id == idx {
            ret.push_str(format!("{}{}{}", Invert, ch, Reset).as_str());
            peekable.next();
        } else {
            ret.push(ch);
        }
    }

    ret
}
