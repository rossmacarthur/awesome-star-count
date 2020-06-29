use std::env;
use std::iter::Peekable;

use anyhow::{Context, Result};
use pulldown_cmark::{CowStr, Event, LinkType, Parser, Tag};
use serde::Deserialize;
use serde::Serialize;
use url::Url;

#[derive(Debug, Default, PartialEq)]
struct Node {
    title: String,
    url: Option<Url>,
    children: Option<Vec<Node>>,
}

impl Node {
    fn push(&mut self, node: Node) {
        self.children.get_or_insert(vec![]).push(node);
    }

    fn append(&mut self, mut nodes: Vec<Node>) {
        match &mut self.children {
            Some(children) => children.append(&mut nodes),
            children @ None => *children = Some(nodes),
        }
    }
}

fn into_sections(current_level: u32, mut parser: &mut Peekable<Parser>) -> Vec<Node> {
    let mut sections: Vec<Node> = Vec::new();

    loop {
        match parser.peek() {
            Some(Event::Start(Tag::Heading(level))) => {
                // going back to a bigger heading
                if *level < current_level {
                    break;
                // going to a smaller heading (need to recurse immediately)
                } else if *level > current_level {
                    // This can happen if we jump heading levels, so create an implicit section
                    // for it.
                    if sections.len() == 0 {
                        sections.push(Node {
                            title: "(implicit)".to_string(),
                            url: None,
                            children: None,
                        })
                    }
                    let section = sections.last_mut().unwrap();
                    section.append(into_sections(current_level + 1, &mut parser));
                    continue;
                } else {
                    // consume this heading
                    parser.next().unwrap();
                    let (title, url) = match parser.next().unwrap() {
                        Event::Text(CowStr::Borrowed(title)) => (title.to_string(), None),
                        Event::Start(Tag::Link(LinkType::Inline, link, title)) => {
                            let title = match parser.next() {
                                Some(Event::Text(title)) => title.clone().into_string(),
                                _ => title.to_string(),
                            };
                            (title, Url::parse(&link).ok())
                        }
                        event => todo!("heading with event: {:?}", event),
                    };

                    // consume until end tag
                    loop {
                        match parser.next() {
                            Some(Event::End(_)) => break,
                            _ => continue,
                        }
                    }

                    // append this heading to the sections
                    sections.push(Node {
                        title,
                        url,
                        children: None,
                    });
                }
            }
            Some(Event::Start(Tag::Item)) => {
                // consume this bullet point
                parser.next().unwrap();
                let (title, url) = match parser.next().unwrap() {
                    Event::Start(Tag::Link(LinkType::Inline, link, title)) => {
                        let title = match parser.next() {
                            Some(Event::Text(title)) => title.clone().into_string(),
                            _ => title.to_string(),
                        };
                        (title, Url::parse(&link).ok())
                    }
                    _ => continue,
                };

                // consume until end tag
                loop {
                    match parser.next() {
                        Some(Event::End(_)) => break,
                        _ => continue,
                    }
                }

                // append this item to the previous node
                match sections.last_mut() {
                    Some(section) => {
                        section.push(Node {
                            title,
                            url,
                            children: None,
                        });
                    }
                    None => todo!("tag without previous section"),
                }
            }
            Some(_) => {
                parser.next();
                continue;
            }
            None => break,
        }
    }

    sections
}

fn parse(url: Url) -> Result<Vec<Node>> {
    let contents = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    let mut parser = Parser::new(&contents).peekable();
    let sections = into_sections(1, &mut parser);
    Ok(sections)
}

fn print_tree(indent: u32, mut sections: Vec<FilteredNode>) {
    sections.sort_by_key(|n| match n {
        FilteredNode::Section { .. } => std::u64::MAX,
        FilteredNode::Item { stars, .. } => stars.unwrap_or(0),
    });
    sections.reverse();

    for node in sections.into_iter().take(10) {
        let stars = match node.stars() {
            Some(stars) => format!(" {} ★ {}", node.url().unwrap(), stars),
            None => String::new(),
        };
        println!("{:3$}{}:{}", "", node.title(), stars, indent as usize);
        if let FilteredNode::Section { children, .. } = node {
            print_tree(indent + 4, children);
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum FilteredNode {
    Section {
        title: String,
        url: Option<Url>,
        children: Vec<FilteredNode>,
    },
    Item {
        title: String,
        url: Url,
        stars: Option<u64>,
    },
}

impl FilteredNode {
    fn title(&self) -> &str {
        match self {
            Self::Section { title, .. } => &title,
            Self::Item { title, .. } => &title,
        }
    }

    fn url(&self) -> Option<&Url> {
        match self {
            Self::Section { url, .. } => url.as_ref(),
            Self::Item { url, .. } => Some(&url),
        }
    }

    fn stars(&self) -> Option<u64> {
        match self {
            Self::Item { stars, .. } => *stars,
            _ => None,
        }
    }
}

fn get_client() -> Result<reqwest::blocking::Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        format!("token {}", env::var("GITHUB_TOKEN")?).parse()?,
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:77.0) Gecko/20100101 Firefox/77.0"
            .parse()?,
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

fn get_stars(client: &reqwest::blocking::Client, url: &Url) -> Result<u64> {
    let mut split = url.path().rsplit('/');
    let repo = split.next().unwrap();
    let owner = split.next().unwrap();
    let api = format!("https://api.github.com/repos/{}/{}", owner, repo);
    eprintln!("GET {}", api);
    let resp: serde_json::Value = client.get(&api).send()?.error_for_status()?.json()?;
    let stars = resp.get("stargazers_count").unwrap().as_u64().unwrap();
    Ok(stars)
}

fn filtered_nodes(
    client: &reqwest::blocking::Client,
    sections: Vec<Node>,
) -> Result<Vec<FilteredNode>> {
    let mut filtered = Vec::new();

    for node in sections {
        let Node {
            title,
            url,
            children,
        } = node;

        let children = match children {
            Some(children) => {
                let filtered_children = filtered_nodes(client, children)?;
                if filtered_children.len() == 0 {
                    None
                } else {
                    Some(filtered_children)
                }
            }
            None => None,
        };

        if children.is_some()
            || url
                .as_ref()
                .and_then(|u| u.host_str().map(|s| s.ends_with("github.com")))
                .unwrap_or(false)
        {
            filtered.push(match children {
                Some(children) => FilteredNode::Section {
                    title,
                    url,
                    children,
                },
                None => {
                    let url = url.unwrap();
                    let stars = get_stars(client, &url).ok();
                    FilteredNode::Item { title, url, stars }
                }
            });
        }
    }

    Ok(filtered)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let url = Url::parse(&args[1])?;
    eprintln!("URL: {}", url);
    let sections = parse(url).context("failed to parse URL contents")?;
    let client = get_client().context("failed to build HTTP client")?;
    let filtered = filtered_nodes(&client, sections).context("failed to filter nodes")?;
    print_tree(1, filtered);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test(contents: &str, sections: &[Node]) {
        let mut parser = Parser::new(contents).peekable();
        println!("{:?}", parser.clone().collect::<Vec<_>>());
        assert_eq!(into_sections(1, &mut parser), sections,);
    }

    #[test]
    fn basic() {
        run_test(
            "# Heading\n",
            &[Node {
                title: "Heading".to_string(),
                ..Default::default()
            }],
        );
    }

    #[test]
    fn nested() {
        run_test(
            "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\n## Heading 4",
            &[Node {
                title: "Heading 1".to_string(),
                children: Some(vec![
                    Node {
                        title: "Heading 2".to_string(),
                        children: Some(vec![Node {
                            title: "Heading 3".to_string(),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    },
                    Node {
                        title: "Heading 4".to_string(),
                        ..Default::default()
                    },
                ]),
                ..Default::default()
            }],
        );
    }
}
