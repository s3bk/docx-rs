use std::fs::File;

use docx::DocxFile;
use zip::ZipArchive;

fn main() {
    env_logger::init();

    let args: Vec<_> = std::env::args_os().collect();
    let reader = File::open(args.get(1).expect("no input file")).unwrap();

    let mut zip = ZipArchive::new(reader).unwrap();
    let file = DocxFile::from_zip(&mut zip).unwrap();
    let _doc = file.parse().unwrap();
}