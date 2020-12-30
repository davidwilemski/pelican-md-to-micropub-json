use std::collections::HashMap;

#[macro_use]
extern crate clap;

use std::io::{self, BufRead, Read};

use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
struct PostContent {
    markdown: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PostProperties {
    name: Vec<String>,

    #[serde(rename = "mp-slug")]
    slug: Vec<String>,

    content: Vec<PostContent>,

    published: Vec<String>,

    category: Vec<String>,

    #[serde(flatten)]
    extra: HashMap<String, Vec<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    #[serde(rename = "type")]
    post_type: Vec<String>,
    properties: PostProperties,
}

#[derive(Debug, Error)]
enum PostError {
    #[error("Field '{0}' is missing. It is a required field to build a Post.")]
    MissingField(String),
}

struct PostBuilder {
    name: Option<String>,
    slug: Option<String>,
    content: Option<String>,
    published: Option<String>,
    categories: Option<Vec<String>>,
}

impl PostBuilder {
    fn new() -> Self {
        PostBuilder {
            name: None,
            slug: None,
            content: None,
            published: None,
            categories: None,
        }
    }

    fn build(self) -> Result<Post, PostError> {
        let name = self.name.ok_or(PostError::MissingField("Title".into()))?;
        let slug = self.slug.ok_or(PostError::MissingField("Slug".into()))?;
        let content = self
            .content
            .ok_or(PostError::MissingField("Content".into()))?;
        let published = self
            .published
            .ok_or(PostError::MissingField("Date".into()))?;
        let categories = self
            .categories
            .unwrap_or_default()
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        Ok(Post {
            post_type: vec!["h-entry".into()],
            properties: PostProperties {
                name: vec![name],
                slug: vec![slug],
                content: vec![PostContent { markdown: content }],
                published: vec![published],
                category: categories,
                extra: HashMap::new(),
            },
        })
    }

    fn set_name(&mut self, val: String) {
        self.name = Some(val);
    }

    fn set_slug(&mut self, val: String) {
        self.slug = Some(val);
    }

    fn set_published(&mut self, val: String) {
        self.published = Some(val);
    }

    fn set_categories(&mut self, val: Vec<String>) {
        self.categories = Some(val);
    }

    fn set_content(&mut self, val: String) {
        self.content = Some(val)
    }
}

fn main() -> anyhow::Result<()> {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true),
        )
        .get_matches();

    let infile = matches.value_of("INPUT").expect("INPUT IS REQUIRED");
    let stdin = io::stdin();
    let mut reader: io::BufReader<Box<dyn Read>> = match infile {
        "-" => io::BufReader::new(Box::new(stdin.lock())),
        _ => {
            let f = std::fs::File::open(infile)?;
            io::BufReader::new(Box::new(f))
        }
    };

    let mut builder = PostBuilder::new();
    let mut line = String::new();
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 {
            break;
        }

        if line == "\n" {
            break;
        }

        let split: Vec<&str> = line.splitn(2, ":").collect::<Vec<&str>>();
        let split_slice: &[&str] = split.as_slice();

        match split_slice {
            ["Title", val] => {
                builder.set_name(val.trim().into());
            }
            ["Slug", val] => {
                builder.set_slug(val.trim().into());
            }
            ["Date", val] => {
                builder.set_published(val.trim().into());
            }
            ["Tags", val] => {
                builder.set_categories(
                    val.trim()
                        .split(",")
                        .map(|t| t.trim().into())
                        .collect::<Vec<String>>(),
                );
            }
            _ => (),
        };

        line.clear();
    }

    let mut content = String::new();
    // We don't super care if the post doesn't have content (although it should)
    let _ = reader.read_to_string(&mut content)?;
    builder.set_content(content);

    let post = builder.build()?;
    println!("{}", serde_json::to_string(&post)?);

    Ok(())
}
