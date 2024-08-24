mod entry;

pub use entry::Entry;
use lazy_static::lazy_static;
use markdown::mdast;
use regex::Regex;

lazy_static! {
    static ref DATE_RE: Regex = Regex::new(r"\b\d{4}-\d{2}-\d{2}\b").unwrap();
}

#[derive(Debug, PartialEq)]
pub struct Log(pub Vec<Entry>);

impl Log {
    pub fn parse(input: &str) -> Self {
        let ast = parse_md(input);
        let Some(nodes) = ast.children() else {
            return Self(vec![]);
        };
        let mut date: Option<chrono::NaiveDate> = None;
        let mut entries: Vec<Entry> = vec![];
        for node in nodes.iter() {
            if let mdast::Node::Heading(_) = node {
                date = parse_heading(&node.to_string());
                continue;
            }
            let mdast::Node::List(mdast::List { children, .. }) = node else {
                continue;
            };
            let Some(ref date) = date else {
                continue;
            };
            for list_item in children.iter() {
                let Some(item_text) = list_item.children().unwrap().first() else {
                    continue;
                };
                if let Some(entry) = Entry::parse(&item_text.to_string(), date) {
                    entries.push(entry);
                };
            }
        }
        entries.sort();
        Self(entries)
    }
}

fn parse_heading(s: &str) -> Option<chrono::NaiveDate> {
    let cap = DATE_RE.captures(s)?;
    chrono::NaiveDate::parse_from_str(&cap[0], "%Y-%m-%d").ok()
}

fn parse_md(input: &str) -> mdast::Node {
    let opts = &markdown::ParseOptions::default();
    markdown::to_mdast(input, opts).unwrap()
}
