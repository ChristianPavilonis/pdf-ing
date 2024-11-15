use std::{f32, fs::File, io::BufWriter};

use printpdf::*;

fn main() {
    generate_pdf(DocConfig::new(215.9, 279.4), vec![
        Text::new("Hello\nworld", 12.0, (16.0, 10.0)),
        Text::new("Foobar", 18.0, (16.0, 25.0)),
    ]);
}


struct DocConfig {
    width: f32,
    height: f32,
}

impl DocConfig {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
        }
    }
}

struct Text {
    content: String,
    font_size: f32,
    cursor_pos: (f32, f32),
    line_height: f32,
}

impl Text {
    pub fn new(content: &str, font_size: f32, cursor_pos: (f32, f32)) -> Self {
        Self {
            content: content.to_string(),
            font_size,
            cursor_pos,
            ..Default::default()
        }
    }
}
impl Default for Text {
    fn default() -> Self {
        Self {
            content: String::new(),
            font_size: 14.0,
            cursor_pos: (14.0, 14.0),
            line_height: 18.0,
        }
    }
}

fn generate_pdf(doc_config: DocConfig, blocks: Vec<Text>) {
    let (doc, pi, li) = PdfDocument::new("Pdf", Mm(doc_config.width), Mm(doc_config.height), "L1");
    let page = doc.get_page(pi);
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .expect("not sure this will ever fail");

    for block in blocks {
        let layer = page.add_layer("");

        layer.begin_text_section();
        layer.set_line_height(block.line_height);

        layer.set_text_cursor(Mm(block.cursor_pos.0), Mm(doc_config.height - block.cursor_pos.1));

        let lines = block.content.split("\n");

        for line in lines {
            layer.set_font(&font, block.font_size);
            layer.write_text(line, &font);
            layer.add_line_break();
        }

        layer.end_text_section();
    }

    let file = File::create("out/test.pdf").unwrap();

    doc.save(&mut BufWriter::new(file)).unwrap();
}
