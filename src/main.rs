use std::iter::Peekable;

use anyhow::Result;
use pulldown_cmark::{CowStr, Event, LinkType, Parser, Tag};

#[derive(Debug, Default, PartialEq)]
struct Node {
    title: String,
    url: Option<String>,
    children: Option<Vec<Node>>,
}

impl Node {
    fn push(&mut self, node: Node) {
        self.children.get_or_insert(vec![]).push(node);
    }

    fn append(&mut self, mut nodes: Vec<Node>) {
        match &mut self.children {
            Some(children) => children.append(&mut nodes),
            None => self.children = Some(nodes),
        }
    }
}

fn into_sections(current_level: u32, mut parser: &mut Peekable<Parser>) -> Vec<Node> {
    let mut sections: Vec<Node> = Vec::new();

    loop {
        match parser.peek() {
            Some(Event::Start(Tag::Heading(level))) => {
                if *level < current_level {
                    // going back to a bigger heading
                    break;
                } else if *level > current_level {
                    // going to a smaller heading (need to recurse immediately)
                    match sections.last_mut() {
                        Some(section) => {
                            section.append(into_sections(current_level + 1, &mut parser));
                        }
                        None => todo!(),
                    }
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
                            (title, Some(link.into_string()))
                        }
                        _ => todo!(),
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
                        (title, Some(link.into_string()))
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
                    None => todo!(),
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

fn parse(url: &str) -> Result<Vec<Node>> {
    let contents = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    let mut parser = Parser::new(&contents).peekable();
    let sections = into_sections(1, &mut parser);
    Ok(sections)
}

fn print_tree(indent: u32, sections: Vec<Node>) {
    for node in sections {
        let Node {
            title,
            url,
            children,
        } = node;

        println!(
            "{:3$}{}: {}",
            "",
            title,
            url.unwrap_or(String::from("")),
            indent as usize
        );

        if let Some(children) = children {
            print_tree(indent + 4, children);
        }
    }
}

fn main() -> Result<()> {
    let url = "https://github.com/unixorn/awesome-zsh-plugins/raw/master/README.md";
    let sections = parse(url)?;
    print_tree(0, sections);
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

    fn with_links() {}
}
