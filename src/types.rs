pub type Fonts = Vec<Font>;

pub struct Font {
  pub postscript: String,
  pub family: String,
  pub id: String,
  pub style: String,
  pub weight: u16,
  pub stretch: u16,
  pub italic: bool,
}
