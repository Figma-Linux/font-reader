extern crate binary_reader;

mod formats;
mod types;

use binary_reader::BinaryReader;
use formats::{ttc, ttf};
use std::fs::File;

fn main() {
  let path_ttf = "/usr/share/fonts/volkhov/Volkhov-BoldItalic.ttf";
  let path_ttc = "/usr/share/fonts/users/PTSerifCaption.ttc";
  let path_otf = "/usr/share/fonts/opentype/malayalam/Manjari-Bold.otf";

  let mut ttf_file_2 = File::open(path_ttf).expect("Cannot read ttf file");
  let mut ttc_file_2 = File::open(path_ttc).expect("Cannot read ttc file");
  let mut otf_file_2 = File::open(path_otf).expect("Cannot read otf file");

  let mut ttf_binary_2 = BinaryReader::from_file(&mut ttf_file_2);
  let mut ttf_reader = ttf::TTF::new(&mut ttf_binary_2, 0);

  match ttf_reader.get_data() {
    Ok(data) => println!(
      "TTF Data: {} - {} - {}",
      data[0].id, data[0].postscript, data[0].weight
    ),
    Err(err) => println!("Read TTF font error: {}", err),
  }

  let mut ttc_binary_2 = BinaryReader::from_file(&mut ttc_file_2);
  let ttc_reader = ttc::TTC::new(&mut ttc_binary_2);

  match ttc_reader.get_data() {
    Ok(data) => println!(
      "TTC Data: {} - {} - {}",
      data[0].id, data[0].postscript, data[0].weight
    ),
    Err(err) => println!("Read TTC font error: {}", err),
  }

  let mut otf_binary_2 = BinaryReader::from_file(&mut otf_file_2);
  let mut otf_reader = ttf::TTF::new(&mut otf_binary_2, 0);

  match otf_reader.get_data() {
    Ok(data) => println!(
      "OTF Data: {} - {} - {}",
      data[0].id, data[0].postscript, data[0].weight
    ),
    Err(err) => println!("Read OTF font error: {}", err),
  }
}
