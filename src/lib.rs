use lazy_static::lazy_static;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::{Error, Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
use std::path::{Path,PathBuf};

pub mod script;

lazy_static! {
    static ref RE: Regex = Regex::new("^\\{\\{#tutorial (.*)\\}\\}$").unwrap();
}

/// A no-op preprocessor.
pub struct BashTutorial;

impl BashTutorial {
    pub fn new() -> BashTutorial {
        BashTutorial
    }
}

impl Preprocessor for BashTutorial {
    fn name(&self) -> &str {
        "bash-tutorial"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
        let src_dir = ctx.root.join(&ctx.config.book.src);

        book.for_each_mut(|section: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = section {
                if let Some(ref _source) = chapter.path {
                    let content = BashTutorial::add_tutorial(&src_dir, chapter)
                            .expect("error adding tutorial in chapter {chapter}");
                    chapter.content = content;
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

impl BashTutorial {
    fn add_tutorial(root: &PathBuf, chapter: &mut Chapter) -> Result<String> {
        add_tutorial(root, &chapter.content)
    }
}

fn add_tutorial(root: &PathBuf, content: &str) -> Result<String> {
    let v = content
        .lines()
        .map(|l| {
            if !RE.is_match(l) {
                return l.to_string();
            }

            let f = l
                .trim_start_matches("{{#tutorial ")
                .trim_end_matches("}}");

            let sf = Path::new(f);
            let v = root.join(&sf);
            if v.exists() {
                let fp = v.as_path().to_str().expect("error joining file");
                return script::parse(fp).to_string();
            }
            return l.to_string();
        })
        .collect::<Vec<String>>()
        .join("\n");
    Ok(v)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nop_preprocessor_run() {
        let input_json = r##"[
            {
                "root": ".",
                "config": {
                    "book": {
                        "authors": ["AUTHOR"],
                        "language": "en",
                        "multilingual": false,
                        "src": ".",
                        "title": "TITLE"
                    },
                    "preprocessor": {
                        "nop": {}
                    }
                },
                "renderer": "html",
                "mdbook_version": "0.4.21"
            },
            {
                "sections": [
                    {
                        "Chapter": {
                            "name": "Chapter 1",
                            "content": "{{#tutorial data/test.sh}}\n",
                            "number": [1],
                            "sub_items": [],
                            "path": "chapter_1.md",
                            "source_path": "chapter_1.md",
                            "parent_names": []
                        }
                    }
                ],
                "__non_exhaustive": null
            }
        ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let output_json = r##"[
            {
                "root": ".",
                "config": {
                    "book": {
                        "authors": ["AUTHOR"],
                        "language": "en",
                        "multilingual": false,
                        "src": "src",
                        "title": "TITLE"
                    },
                    "preprocessor": {
                        "nop": {}
                    }
                },
                "renderer": "html",
                "mdbook_version": "0.4.21"
            },
            {
                "sections": [
                    {
                        "Chapter": {
                            "name": "Chapter 1",
                            "content": "1. Test\n\n\t```bash\n\techo \"test\"\n\t```\n",
                            "number": [1],
                            "sub_items": [],
                            "path": "chapter_1.md",
                            "source_path": "chapter_1.md",
                            "parent_names": []
                        }
                    }
                ],
                "__non_exhaustive": null
            }
        ]"##;
        let output_json = output_json.as_bytes();

        let (_ctx, expected_book) = mdbook::preprocess::CmdPreprocessor::parse_input(output_json).unwrap();
        let result = BashTutorial::new().run(&ctx, book);
        assert!(result.is_ok());

        // The nop-preprocessor should not have made any changes to the book content.
        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }
}
