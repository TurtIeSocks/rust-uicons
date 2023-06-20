use serde_json;
use std::fs;

const STYLES: [&str; 2] = ["artificial", "safe"];
const FORMATS: [&str; 3] = ["svg", "png", "webp"];

pub fn create_directories() {
    for style in STYLES.iter() {
        for format in FORMATS.iter() {
            match fs::create_dir_all(format!("./uicons/{}/{}/pokemon", style, format)) {
                Ok(_) => println!("Created {}/{} directory", style, format),
                Err(_) => println!("Unable to create directory"),
            }
        }
    }
}

pub fn create_jsons() {
    for style in STYLES.iter() {
        for format in FORMATS.iter() {
            let file_names =
                get_string_filenames(&format!("./uicons/{}/{}/pokemon", style, format), true);

            let mut json = String::new();
            json.push_str("[");
            for (i, file_name) in file_names.iter().enumerate() {
                json.push_str(&format!("\"{}\"", file_name));
                if i != file_names.len() - 1 {
                    json.push_str(",");
                }
            }
            json.push_str("]");
            let pretty_json = pretty_json_from_string(&json);
            match fs::write(
                format!("./uicons/{}/{}/pokemon/index.json", style, format),
                pretty_json,
            ) {
                Ok(_) => println!("Created {}/{} json", style, format),
                Err(_) => println!("Unable to create json"),
            }
            let pretty_json = pretty_json_from_string(&format!("{{\"pokemon\": {}}}", json));
            match fs::write(
                format!("./uicons/{}/{}/index.json", style, format),
                pretty_json,
            ) {
                Ok(_) => println!("Created {}/{} json", style, format),
                Err(_) => println!("Unable to create json"),
            }
        }
    }
}

pub fn get_string_filenames(dir: &str, sort: bool) -> Vec<String> {
    let mut list = fs::read_dir(dir)
        .unwrap()
        .filter_map(|file| {
            let file = file.unwrap().file_name().to_string_lossy().to_string();
            if file.ends_with(".json") {
                None
            } else {
                Some(file)
            }
        })
        .collect::<Vec<String>>();
    if sort {
        list.sort_by(|a, b| {
            let a = a.split(".").nth(0).unwrap();
            let a = a.split("_").nth(0).unwrap();
            let b = b.split(".").nth(0).unwrap();
            let b = b.split("_").nth(0).unwrap();
            let a = match a.parse::<u16>() {
                Ok(a) => a,
                Err(err) => {
                    println!("{} {}", a, err);
                    0
                }
            };
            let b = match b.parse::<u16>() {
                Ok(b) => b,
                Err(err) => {
                    println!("{} {}", b, err);
                    0
                }
            };
            a.cmp(&b)
        });
    }
    list
}

pub fn pretty_json_from_string(json: &String) -> String {
    let json: serde_json::Value = serde_json::from_str(json).unwrap();
    serde_json::to_string_pretty(&json).unwrap()
}
