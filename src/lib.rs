use std::{error::Error, fs::File, io::BufWriter, path::Path};

use printpdf::*;
use thiserror::Error;

pub struct Doc {
    width: f32,
    height: f32,
    doc: PdfDocumentReference,
    pi: PdfPageIndex,
    font: IndirectFontRef,
}

impl Doc {
    pub fn new(width: f32, height: f32) -> Self {
        let (doc, pi, _) = PdfDocument::new("Pdf", Mm(width), Mm(height), "L1");
        let font = doc
            .add_builtin_font(BuiltinFont::Helvetica)
            .expect("You don't have Helvetica for some reason");
        Self {
            width,
            height,
            doc,
            pi,
            font,
        }
    }

    pub fn add_layer(&self) -> PdfLayerReference {
        self.doc.get_page(self.pi).add_layer("")
    }
}

pub enum PdfBlock<P: AsRef<Path>> {
    TextSection {
        nodes: Vec<TextNode>,
        pos: (f32, f32),
    },
    Image {
        path: P,
        dpi: f32,
        pos: (f32, f32),
    },
}

impl<P: AsRef<Path>> PdfBlock<P> {
    pub fn render(&self, layer: &PdfLayerReference, doc: &Doc) -> Result<(), Box<dyn Error>> {
        match self {
            PdfBlock::TextSection { nodes, pos } => {
                layer.begin_text_section();
                layer.set_text_cursor(Mm(pos.0), Mm(doc.height - pos.1));
                for node in nodes.iter() {
                    layer.set_line_height(node.line_height);
                    let lines = node.content.split('\n');
                    for line in lines {
                        layer.set_font(&doc.font, node.font_size);
                        layer.write_text(line, &doc.font);
                        layer.add_line_break();
                    }
                }
                layer.end_text_section();
            }
            PdfBlock::Image { path, dpi, pos } => {
                let reader = image_crate::io::Reader::open(path)?;
                let image = reader.decode()?;

                let image = printpdf::Image::from_dynamic_image(&image);
                let image_width: Mm = image.image.width.into_pt(*dpi).into();
                let image_height: Mm = image.image.height.into_pt(*dpi).into();
                image.add_to_layer(
                    layer.clone(),
                    ImageTransform {
                        translate_x: Some(Mm(pos.0 - image_width.0)),
                        translate_y: Some(Mm(doc.height - pos.1 - image_height.0)),
                        dpi: Some(*dpi),
                        ..ImageTransform::default()
                    },
                );
            }
        }
        Ok(())
    }
}

pub struct TextNode {
    pub content: String,
    pub font_size: f32,
    pub line_height: f32,
}

pub fn generate_pdf<P: AsRef<Path>, I: AsRef<Path>>(
    doc: Doc,
    blocks: Vec<PdfBlock<I>>,
    path: P,
) -> Result<(), PdfingError> {
    for block in blocks {
        let layer = doc.add_layer();
        block
            .render(&layer, &doc)
            .map_err(|e| PdfingError { msg: e.to_string() })?;
    }
    let file = File::create(path).map_err(|e| PdfingError { msg: e.to_string() })?;
    doc.doc
        .save(&mut BufWriter::new(file))
        .map_err(|e| PdfingError { msg: e.to_string() })?;

    Ok(())
}

#[derive(Error, Debug)]
#[error("{msg}")]
pub struct PdfingError {
    msg: String,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    #[test]
    fn test_generate_pdf() {
        let doc = Doc::new(215.9, 279.4);
        let blocks: Vec<PdfBlock<&str>> = vec![
            PdfBlock::Image {
                path: "./quantum-joe.jpeg",
                dpi: 300.0,
                pos: (200.0, 20.0),
            },
            PdfBlock::TextSection {
                nodes: vec![
                    TextNode {
                        content: "Serendipitous Reflections".to_string(),
                        font_size: 24.0,
                        line_height: 24.0,
                    },
                    TextNode {
                        content: "In the grand fooo of life's comedy,\n\
                        the keen eye discerns humor lurking amidst the everyday trivialities."
                            .to_string(),
                        font_size: 12.0,
                        line_height: 14.0,
                    },
                ],
                pos: (20.0, 40.0),
            },
            PdfBlock::TextSection {
                nodes: vec![
                    TextNode {
                        content: "Tapestry of Time".to_string(),
                        font_size: 24.0,
                        line_height: 24.0,
                    },
                    TextNode {
                        content: "Behold! The threads of fate weaving storied pasts\n\
                        with present musings, like a fashion show designed by Cosmo Kramer."
                            .to_string(),
                        font_size: 12.0,
                        line_height: 14.0,
                    },
                ],
                pos: (20.0, 100.0),
            },
        ];

        generate_pdf(doc, blocks, PathBuf::from("out/test.pdf")).unwrap();
    }
}
