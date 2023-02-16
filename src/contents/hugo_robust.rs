use super::{Contents, ContentsFormat};

// use comrak::{format_commonmark, parse_document, Arena, ComrakOptions};

pub struct HugoRobust {}
impl ContentsFormat for HugoRobust {
    fn format_body(&self, body: String) -> String {
        /*
        let text = "# ds af\nf klj;ad\n## sfd";
        let arena = Arena::new();
        let root = parse_document(&arena, text, &ComrakOptions::default());

        let mut out = vec![];
        format_commonmark(root, &ComrakOptions::default(), &mut out).unwrap();

        println!("{}", String::from_utf8(out).unwrap());
        */
        return body;
    }
    fn target_dirname(&self) -> String {
        return "content".to_string();
    }
}

impl Contents<HugoRobust> {
    pub fn new(output_dirpath: String) -> Self {
        Self {
            contents_format: HugoRobust {},
            output_dirpath,
        }
    }
}
