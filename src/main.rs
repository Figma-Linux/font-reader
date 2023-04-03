extern crate binary_reader;
extern crate font_reader;

mod formats;
mod types;

use font_reader::read;

fn main() {
  let paths = vec![
    // "/home/ruut/.local/share/fonts/Segoe/SegoeUI-BoldItalic.ttf",
    // "/home/ruut/.local/share/fonts/Segoe/SegoeUI-Bold.ttf",
    // "/home/ruut/.local/share/fonts/Segoe/SegoeUI-Italic.ttf",
    // "/home/ruut/.local/share/fonts/Segoe/SegoeUI-Light.ttf",
    // "/home/ruut/.local/share/fonts/Segoe/SegoeUI-SemiBold.ttf",
    // "/home/ruut/.local/share/fonts/Segoe/SegoeUI.ttf",
    // "/home/ruut/.local/share/fonts/otf/expletus-sans-italic.otf",
    // "/home/ruut/.local/share/bad_fonts/Portico Light Rough.otf",
    // "/home/ruut/.local/share/fonts/variable/AngstVF.ttf",
    // "/home/ruut/.local/share/fonts/variable/RobotoFlex[slnt,wdth,wght,opsz].ttf",
    // "/home/ruut/.local/share/fonts/otf/fundamental-brigade-scvhlank.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoBoldItalic.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoBold.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoHeavyItalic.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoHeavy.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoLightItalic.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoLight.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoMediumItalic.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoMedium.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoRegularItalic.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoRegular.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoSemiboldItalic.otf",
    // "/home/ruut/.local/share/fonts/otf/SFMonoSemibold.otf",
    // "/usr/share/fonts/truetype/Gargi/Gargi.ttf",
    // "/usr/share/fonts/users/DevanagariMT.ttc",
    // "/usr/share/fonts/truetype/freefont/FreeMonoOblique.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeSerifBoldItalic.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeSerifBold.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeMonoBoldOblique.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeSans.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeSerifItalic.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeMonoBold.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeSansBold.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeSansBoldOblique.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeMono.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeSansOblique.ttf",
    // "/usr/share/fonts/truetype/freefont/FreeSerif.ttf",
    "/usr/share/fonts/truetype/pagul/Pagul.ttf",
  ];

  for path in paths {
    match read(path) {
      Ok(data) => {
        println!("Font {} | Font count: {}", path, data.len(),);

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
