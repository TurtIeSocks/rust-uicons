use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct Form {
    name: Option<String>,
    types: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pokemon {
    pokedex_id: u16,
    pub forms: HashMap<String, Form>,
    default_form_id: u16,
    types: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Masterfile {
    pub pokemon: HashMap<String, Pokemon>,
}

#[derive(Debug, Clone)]
pub struct PokemonColor<'a> {
    pub id: u16,
    pub form: Option<u16>,
    pub color_1: &'a str,
    pub color_2: &'a str,
}

impl<'a> PokemonColor<'a> {
    pub fn get_filename(&self) -> String {
        format!(
            "{}{}",
            self.id,
            if let Some(form) = self.form {
                format!("_f{}", form)
            } else {
                "".to_string()
            }
        )
    }
}

const DEFAULT_COLOR: &str = "#fff";

impl Masterfile {
    pub fn get_pokemon_colors(&self) -> Vec<PokemonColor> {
        let color_map: HashMap<&str, &str> = HashMap::from([
            ("Normal", "#A8A877"),
            ("Fire", "#EF8030"),
            ("Water", "#6390F0"),
            ("Electric", "#F8CF30"),
            ("Grass", "#78C84F"),
            ("Ice", "#98D8D8"),
            ("Fighting", "#C03028"),
            ("Flying", "#A890F0"),
            ("Poison", "#9F409F"),
            ("Ground", "#E0C068"),
            ("Psychic", "#F85788"),
            ("Rock", "#B8A038"),
            ("Bug", "#A8B720"),
            ("Dragon", "#7038F8"),
            ("Ghost", "#705898"),
            ("Dark", "#705848"),
            ("Steel", "#B8B8D0"),
            ("Fairy", "#EE99AC"),
        ]);
        let mut unique_combos = vec![];

        self.pokemon.values().for_each(|pokemon| {
            let color_1 = if let Some(first_type) = pokemon.types.first() {
                color_map.get(&first_type[..]).unwrap()
            } else {
                DEFAULT_COLOR
            };
            let color_2 = if let Some(second_type) = pokemon.types.last() {
                color_map.get(&second_type[..]).unwrap()
            } else {
                color_1.clone()
            };
            unique_combos.push(PokemonColor {
                id: pokemon.pokedex_id,
                form: None,
                color_1,
                color_2,
            });
            pokemon.forms.iter().for_each(|(form_id, form)| {
                if let Some(form_types) = form.types.as_ref() {
                    let color_1 = if let Some(first_type) = form_types.first() {
                        color_map.get(&first_type[..]).unwrap()
                    } else {
                        DEFAULT_COLOR
                    };
                    let color_2 = if let Some(second_type) = form_types.last() {
                        color_map.get(&second_type[..]).unwrap()
                    } else {
                        color_1.clone()
                    };
                    unique_combos.push(PokemonColor {
                        id: pokemon.pokedex_id,
                        form: Some(form_id.parse::<u16>().unwrap()),
                        color_1,
                        color_2,
                    });
                }
            })
        });
        unique_combos
    }
}

pub async fn get_masterfile() -> Result<Masterfile, reqwest::Error> {
    reqwest::get("https://raw.githubusercontent.com/WatWowMap/Masterfile-Generator/master/master-latest.json")
      .await?
      .json::<Masterfile>()
      .await
}

fn has_common_substring(a: &str, b: &str, len: usize) -> bool {
    if a.len() < len || b.len() < len {
        return a == b
    }
    for i in 0..=(a.len() - len) {
        if i+len > a.len() {
            println!("A: {} | B: {}", a, b);
        }
        let substring = &a[i..i + len];
        if b.contains(substring) {
            return true;
        }
    }
    false
}

pub fn get_uicons_name(file: &String, masterfile: &HashMap<String, Pokemon>) -> (String, bool) {
    let parts: Vec<&str> = file.split('-').collect();
    let id = parts[0];
    let properties = &parts[2..];

    let mut transformed_name = id.parse::<u32>().unwrap().to_string();
    let mut is_default = false;

    for property in properties {
        let property = property.split('.').nth(0).unwrap();
        if let Some(pokemon) = masterfile.get(id) {
            if let Some((form_id, _)) = pokemon.forms.iter().find(|(_, form)| {
                if let Some(name) = &form.name {
                    has_common_substring(
                        name.to_ascii_lowercase().as_str(),
                        property.to_ascii_lowercase().as_str(),
                        3,
                    )
                } else {
                    false
                }
            }) {
                transformed_name += &format!("_f{}", form_id);
                if pokemon.default_form_id == form_id.to_string().parse::<u16>().unwrap_or(0) {
                    is_default = true;
                };
            }
        }
        if property == "shiny" {
            transformed_name += "_s";
        }
    }
    (transformed_name, is_default)
}
