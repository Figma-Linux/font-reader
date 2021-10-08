extern crate binary_reader;

mod formats;
pub mod types;

use binary_reader::BinaryReader;
use formats::{ttc::TTC, ttf::TTF};
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;
use types::Fonts;

pub fn read(path: &str) -> Result<Fonts, Error> {
  let parsed = Path::new(&path);
  let ext = parsed.extension().unwrap();

  let mut file = File::open(&path)?;
  let mut binary = BinaryReader::from_file(&mut file);

  if ext == "ttf" || ext == "otf" {
    let mut reader = TTF::new(&mut binary, 0);

    match reader.get_data() {
      Ok(data) => return Ok(data),
      Err(err) => return Err(err),
    }
  } else if ext == "ttc" {
    let reader = TTC::new(&mut binary);

    match reader.get_data() {
      Ok(data) => return Ok(data),
      Err(err) => return Err(err),
    }
  }

  Err(Error::new(
    ErrorKind::InvalidInput,
    format!("Invalid font file: {}", path),
  ))
}
