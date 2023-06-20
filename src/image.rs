use image::ImageResult;
use std::{fs, io};
use usvg::{TreeParsing, fontdb::Database, TreeTextToPath};

use crate::pokemon::PokemonColor;

const SVG_TEMPLATE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="128" height="128" viewBox="0 0 128 128">
  <defs>
    <linearGradient id="gradient" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stop-color="{{color_1}}" />
      <stop offset="100%" stop-color="{{color_2}}" />
    </linearGradient>
  </defs>
  <circle cx="64" cy="64" r="62" fill="url(#gradient)" stroke="black" stroke-width="4" />
  <text x="64" y="{{y}}" text-anchor="middle" font-weight="bold" fill="white" font-family="Roboto Condensed" font-size="{{font_size}}px">{{text}}</text>
</svg>"#;

pub fn move_svg(original: &String, file_name: &String) -> io::Result<()> {
    let blob = fs::read(format!("./pokemon-icons/_icons/SVG/{}", original))?;
    fs::write(
        format!("./uicons/artificial/svg/pokemon/{}.svg", file_name),
        blob,
    )
}

pub fn create_svg(pokemon_color: &PokemonColor) -> io::Result<()> {
    let font_size = match pokemon_color.id.to_string().len() {
        1 => 80,
        2 => 60,
        3 => 50,
        _ => 40,
    };
    let y = match pokemon_color.id.to_string().len() {
        1 => 90,
        2 => 84,
        3 => 80,
        _ => 76,
    };
    fs::write(
        format!("./uicons/safe/svg/pokemon/{}.svg", pokemon_color.get_filename(),),
        SVG_TEMPLATE
            .replace("{{color_1}}", pokemon_color.color_1)
            .replace("{{color_2}}", pokemon_color.color_2)
            .replace("{{text}}", &pokemon_color.id.to_string())
            .replace("{{font_size}}", &font_size.to_string())
            .replace("{{y}}", &y.to_string())
            .as_bytes(),
    )
}

pub fn create_png(file_name: &String, style: &str, scale: f32, fonts: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let svg_data = std::fs::read(format!("./uicons/{}/svg/pokemon/{}.svg", style, file_name))?;
    let mut usvg_tree = usvg::Tree::from_data(&svg_data, &usvg::Options::default())?;
    usvg_tree.convert_text(fonts);
    let rtree = resvg::Tree::from_usvg(&usvg_tree);
    if let Some(mut pixmap) = resvg::tiny_skia::Pixmap::new(128, 128) {
        rtree.render(
            usvg::Transform::from_scale(scale, scale),
            &mut pixmap.as_mut(),
        );
        pixmap.save_png(format!("./uicons/{}/png/pokemon/{}.png", style, file_name))?;
        Ok(())
    } else {
        Err("Unable to create pixmap".into())
    }
}

pub fn create_webp(file_name: &String, style: &str) -> ImageResult<()> {
    let img = image::open(format!("./uicons/{}/png/pokemon/{}.png", style, file_name))?;
    img.save(format!(
        "./uicons/{}/webp/pokemon/{}.webp",
        style, file_name
    ))
}
