use std::{
    error::Error,
    f32,
    fs::{File, OpenOptions},
    io::{BufWriter, Cursor, Read},
    path::Path,
};

use image_crate::codecs::jpeg::JpegDecoder;
use printpdf::*;

fn main() {
    let doc = Doc::new(215.9, 279.4);

    let blocks: Vec<Box<dyn Renderable>> = vec![
        Box::new(Image {
            path: "./quantum-joe.jpeg",
            dpi: 300.0,
            pos: (200.0, 20.0),
        }),
        Box::new(TextSection {
            nodes: vec![
                Text {
                    content: "Serendipitous Reflections".to_string(),
                    font_size: 24.0,
                    line_height: 24.0,
                },
                Text {
                    content: "In the grand tapestry of life's comedy, \n\
                the keen eye discerns humor lurking amidst the everyday trivialities."
                        .to_string(),
                    font_size: 12.0,
                    line_height: 14.0,
                },
            ],
            pos: (20.0, 40.0),
        }),
        Box::new(TextSection {
            nodes: vec![
                Text {
                    content: "Tapestry of Time".to_string(),
                    font_size: 24.0,
                    line_height: 24.0,
                },
                Text {
                    content: "Behold! The threads of fate weaving storied pasts \n\
                with present musings, like a fashion show designed by Cosmo Kramer."
                        .to_string(),
                    font_size: 12.0,
                    line_height: 14.0,
                },
            ],
            pos: (20.0, 100.0),
        }),
        Box::new(TextSection {
            nodes: vec![
                Text {
                    content: "Whispers of Evolution".to_string(),
                    font_size: 24.0,
                    line_height: 24.0,
                },
                Text {
                    content: "Unceasing metamorphosis defines our journey, \n\
                linked indelibly to the reverberations of existence—Kramer's vision personified."
                        .to_string(),
                    font_size: 12.0,
                    line_height: 14.0,
                },
            ],
            pos: (20.0, 160.0),
        }),
    ];

    generate_pdf(doc, blocks);
}

struct Doc {
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

trait Renderable {
    fn render(&self, layer: &PdfLayerReference, doc: &Doc) -> Result<(), Box<dyn Error>>;
}

struct TextSection {
    nodes: Vec<Text>,
    pos: (f32, f32),
}

impl Renderable for TextSection {
    fn render(&self, layer: &PdfLayerReference, doc: &Doc) -> Result<(), Box<dyn Error>> {
        layer.begin_text_section();
        layer.set_text_cursor(Mm(self.pos.0), Mm(doc.height - self.pos.1));

        for node in self.nodes.iter() {
            layer.set_line_height(node.line_height);

            let lines = node.content.split("\n");

            for line in lines {
                layer.set_font(&doc.font, node.font_size);
                layer.write_text(line, &doc.font);
                layer.add_line_break();
            }
        }

        layer.end_text_section();
        Ok(())
    }
}

struct Text {
    content: String,
    font_size: f32,
    line_height: f32,
}

// TODO: support more than just jpeg
struct Image<P>
where
    P: AsRef<Path> + Copy,
{
    path: P,
    dpi: f32,
    pos: (f32, f32),
}

impl<P> Renderable for Image<P>
where
    P: AsRef<Path> + Copy,
{
    fn render(&self, layer: &PdfLayerReference, doc: &Doc) -> Result<(), Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(self.path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let bytes: &[u8] = &buffer;

        let mut reader = Cursor::new(bytes);
        let decoder = JpegDecoder::new(&mut reader)?;
        let image = printpdf::Image::try_from(decoder)?;

        let image_width: Mm = image.image.width.into_pt(self.dpi).into();
        let image_height: Mm = image.image.height.into_pt(self.dpi).into();

        image.add_to_layer(
            layer.clone(),
            ImageTransform {
                translate_x: Some(Mm(self.pos.0 - image_width.0)),
                translate_y: Some(Mm(doc.height - self.pos.1 - image_height.0)),
                dpi: Some(self.dpi),
                ..ImageTransform::default()
            },
        );

        Ok(())
    }
}

fn generate_pdf(doc: Doc, blocks: Vec<Box<dyn Renderable>>) {
    for block in blocks {
        let layer = doc.add_layer();

        block.render(&layer, &doc).expect("fail rendering");
    }

    let file = File::create("out/test.pdf").unwrap();

    doc.doc.save(&mut BufWriter::new(file)).unwrap();
}
