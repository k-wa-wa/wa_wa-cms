use std::{fs, fs::File, io::Write, path::Path};

use glob::glob;

pub mod hugo_robust;
use self::hugo_robust::HugoRobust;

struct Contents<T: ContentsFormat> {
    contents_format: T,
    output_dirpath: String,
}

impl<T: ContentsFormat> Contents<T> {
    fn export_contents(&self) {
        for source_filepath in glob("./articles/**/*.md").unwrap().map(|e| e.unwrap()) {
            let body = fs::read_to_string(&source_filepath).unwrap();
            let source_filename = source_filepath.file_name().unwrap().to_str().unwrap();

            let target_dirpath =
                Path::new(&self.output_dirpath).join(self.contents_format.target_dirname());
            fs::create_dir_all(&target_dirpath).unwrap();
            let mut file = File::create(target_dirpath.join(source_filename)).unwrap();
            file.write_all(self.contents_format.format_body(body).as_bytes())
                .unwrap();
        }
    }
}

trait ContentsFormat {
    fn format_body(&self, body: String) -> String;
    fn target_dirname(&self) -> String;
}

pub fn export_zenn_contents2hugo_robust(output_dirpath: String) {
    let c = Contents::<HugoRobust>::new(output_dirpath);
    c.export_contents();
}
