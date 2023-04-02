use crate::types::{Font, Fonts};

use binary_reader::BinaryReader;
use std::io::{Error, ErrorKind};

pub struct TTF<'a> {
  num_of_tables: u16,
  base_offset: usize,
  reader: &'a mut BinaryReader,
}

impl<'a> TTF<'a> {
  pub fn new(reader: &'a mut BinaryReader, offset: usize) -> Self {
    TTF {
      reader,
      base_offset: offset,
      num_of_tables: 0,
    }
  }

  fn read_header(&mut self) -> Result<u16, Error> {
    self.reader.jmp(self.base_offset);

    let u_major_version = self.reader.read_u16()?;
    let u_minor_version = self.reader.read_u16()?;

    if u_major_version != 1 || u_minor_version != 0 {
      self.reader.jmp(self.base_offset);

      let tag = self.reader.read_cstr()?;

      if tag != "OTTO" {
        return Err(Error::new(ErrorKind::InvalidData, "Not a ttf or otf file"));
      }
    }

    let u_num_of_tables = self.reader.read_u16()?;

    self.num_of_tables = u_num_of_tables;

    Ok(u_num_of_tables)
  }

  fn get_table_offset(&mut self, name: String) -> Result<u32, Error> {
    self.reader.jmp(12);

    for _ in 0..self.num_of_tables {
      let table_name_data = match self.reader.read(4) {
        Some(value) => value,
        None => {
          return Err(Error::new(
            ErrorKind::InvalidInput,
            "Cannot read ttf table name",
          ))
        }
      };

      let table_name = match String::from_utf8(table_name_data.to_vec()) {
        Ok(value) => value,
        Err(err) => {
          return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Cannot parse table name bytes into String, err: {}", err),
          ))
        }
      };

      // Skip check sum
      self.reader.adv(4);

      let table_offset = self.reader.read_u32()?;

      // Skip table length
      self.reader.adv(4);

      if table_name == name {
        return Ok(table_offset);
      }
    }

    Err(Error::new(
      ErrorKind::InvalidInput,
      format!("The '{}' table not found", name),
    ))
  }

  pub fn get_data(&mut self) -> Result<Fonts, Error> {
    self.read_header()?;

    let mut offset = self.get_table_offset(String::from("name"))?;

    self.reader.jmp(offset as usize);
    self.reader.adv(2); // Skip uFSelector

    let name_record_count = self.reader.read_u16()?;
    let storage_offset = self.reader.read_u16()?;
    let record_offset = offset + (storage_offset as u32);
    let mut font = Font {
      postscript: String::from(""),
      family: String::from(""),
      id: String::from(""),
      style: String::from(""),
      weight: 400,
      stretch: 5,
      italic: false,
    };

    for _ in 0..name_record_count {
      let platform_id = self.reader.read_u16()?;
      let encoding_id = self.reader.read_u16()?;
      self.reader.adv(2); // skip language id
      let name_id = self.reader.read_u16()?;
      let string_length_id = self.reader.read_u16()?;
      let string_offset_id = self.reader.read_u16()?;
      let prev_offset = self.reader.pos;

      if string_length_id > 0
        && (platform_id == 0 || platform_id == 3)
        && (encoding_id == 0 || encoding_id == 1 || encoding_id == 3)
        && (name_id == 1 || name_id == 2 || name_id == 6 || name_id == 17)
      {
        self
          .reader
          .jmp((record_offset + (string_offset_id as u32)) as usize);
        let mut string_bytes = self
          .reader
          .read(string_length_id as usize)
          .unwrap()
          .to_vec();
        string_bytes.retain(|&i| i != 0);
        let mut value = String::from("");

        match String::from_utf8(string_bytes.to_vec()) {
          Ok(str) => value = str,
          Err(_) => value = String::from(""),
        };

        // Family name
        if name_id == 1 && font.id == "" && value.as_bytes()[0].is_ascii_alphabetic() {
          font.id = String::from(&value);
          font.family = String::from(&value);
        }
        // style name
        if font.style == ""
          && value.as_bytes()[0].is_ascii_alphabetic()
          && (name_id == 2 || name_id == 17)
        {
          font.style = String::from(&value);

          if value.contains("Italic") {
            font.italic = true;
          }
        }
        // postscript name
        if name_id == 6 && font.postscript == "" && value.as_bytes()[0].is_ascii_alphabetic() {
          font.postscript = String::from(&value);
        }

        self.reader.jmp(prev_offset);
      }
    }

    offset = self.get_table_offset(String::from("OS/2"))?;

    self.reader.jmp(offset as usize);
    self.reader.adv(4);

    let weight = self.reader.read_u16()?;

    font.weight = weight;

    Ok(vec![font])
  }
}
