use crate::formats::ttf;
use crate::types::Fonts;

use binary_reader::BinaryReader;
use std::io::{Error, ErrorKind};

pub struct TTC<'a> {
  num_of_fonts: u32,
  reader: &'a mut BinaryReader,
}

impl<'a> TTC<'a> {
  pub fn new(reader: &'a mut BinaryReader) -> Self {
    TTC {
      reader,
      num_of_fonts: 0,
    }
  }

  fn read_header(&mut self) -> Result<u32, Error> {
    let tag = self.reader.read(4).expect("Cannot read TTC file tag");
    let tag_name = String::from_utf8(tag.to_vec()).expect("Cannot parse ttc tag bytes into string");

    if tag_name == "ttcf" {
      let u_major_version = self.reader.read_u16()?;
      let u_minor_version = self.reader.read_u16()?;

      if (u_major_version == 1 || u_major_version == 2) && u_minor_version == 0 {
        let u_num_of_fonts = self.reader.read_u32()?;

        self.num_of_fonts = u_num_of_fonts;

        return Ok(u_num_of_fonts);
      }
    }

    Err(Error::new(ErrorKind::InvalidData, "Not a ttc file"))
  }

  pub fn get_data(mut self) -> Result<Fonts, Error> {
    self.read_header()?;

    let mut fonts: Fonts = vec![];
    let mut last_pos;

    for _ in 0..self.num_of_fonts {
      let font_offset = self.reader.read_u32().unwrap();

      last_pos = self.reader.pos;

      let mut ttf_reader = ttf::TTF::new(self.reader, font_offset as usize);
      let mut font = ttf_reader.get_data()?;

      fonts.push(font.remove(0));

      self.reader.jmp(last_pos);
    }

    Ok(fonts)
  }
}
