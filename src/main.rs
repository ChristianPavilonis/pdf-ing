use std::{f32, fs::File, io::BufWriter};

use printpdf::*;

fn main() {
    generate_pdf(
        DocConfig::new(215.9, 279.4),
        vec![
            Block {
                pos: (16.0, 16.0),
                items: vec![
                    Item::Text {
                        content: "Introduction Header".to_string(),
                        font_size: 16.0,
                        line_height: 24.0,
                    },
                    Item::Text {
                        content: "Content under introduction".to_string(),
                        font_size: 12.0,
                        line_height: 18.0,
                    },
                ],
            },
            Block {
                pos: (16.0, 100.0),
                items: vec![
                    Item::Text {
                        content: "Chapter 1 Header".to_string(),
                        font_size: 16.0,
                        line_height: 24.0,
                    },
                    Item::Text {
                        content: "Content of Chapter 1 delving into the intricacies of the topic."
                            .to_string(),
                        font_size: 12.0,
                        line_height: 18.0,
                    },
                    Item::Text {
                        content: "Continuation with Seinfeld humor...yada yada yada.".to_string(),
                        font_size: 12.0,
                        line_height: 18.0,
                    },
                ],
            },
            Block {
                pos: (16.0, 200.0),
                items: vec![
                    Item::Text {
                        content: "Chapter 2 Header".to_string(),
                        font_size: 16.0,
                        line_height: 24.0,
                    },
                    Item::Text {
                        content: "Content of Chapter 2 exploring new dimensions.".to_string(),
                        font_size: 12.0,
                        line_height: 18.0,
                    },
                ],
            },
        ],
    );
}

struct DocConfig {
    width: f32,
    height: f32,
}

impl DocConfig {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

struct Block {
    items: Vec<Item>,
    pos: (f32, f32),
}

enum Item {
    Text {
        content: String,
        font_size: f32,
        line_height: f32,
    },
}

fn generate_pdf(doc_config: DocConfig, blocks: Vec<Block>) {
    let (doc, pi, li) = PdfDocument::new("Pdf", Mm(doc_config.width), Mm(doc_config.height), "L1");
    let page = doc.get_page(pi);
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .expect("not sure this will ever fail");

    for block in blocks {
        let layer = page.add_layer("");
        layer.begin_text_section();
        layer.set_text_cursor(Mm(block.pos.0), Mm(doc_config.height - block.pos.1));

        for item in block.items {
            match item {
                Item::Text {
                    content,
                    font_size,
                    line_height,
                } => {
                    layer.set_line_height(line_height);

                    let lines = content.split("\n");

                    for line in lines {
                        layer.set_font(&font, font_size);
                        layer.write_text(line, &font);
                        layer.add_line_break();
                    }
                }
            }
        }

        layer.end_text_section();
    }

    let file = File::create("out/test.pdf").unwrap();

    doc.save(&mut BufWriter::new(file)).unwrap();
}
