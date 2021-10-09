extern crate binary_reader;
extern crate font_reader;

mod formats;
mod types;

use font_reader::read;

fn main() {
  let paths = vec![
    "/usr/share/fonts/volkhov/Volkhov-BoldItalic.ttf",
    "/usr/share/fonts/users/PTSerifCaption.ttc",
    "/usr/share/fonts/opentype/malayalam/Manjari-Bold.otf",
    "/home/ruut/.local/share/bad_fonts/Portico Light Rough.otf",
  ];

  for path in paths {
    match read(path) {
      Ok(data) => {
        println!("Font {}\nFont count: {}", path, data.len(),);

        for font in data {
          println!(
            " family: {}\n id: {}\n style: {}\n postscript: {}\n weight: {}\n stretch: {}\n italic: {}\n",
            font.family,
            font.id,
            font.style,
            font.postscript,
            font.weight,
            font.stretch,
            font.italic
          )
        }
      }
      Err(err) => println!("Read '{}' font error: {}", path, err),
    }
  }
}
