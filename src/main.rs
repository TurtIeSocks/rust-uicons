use std::thread;

mod image;
mod pokemon;
mod uicons;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let time = std::time::Instant::now();
    let cpus = num_cpus::get();
    println!("Found {} cores", cpus);

    uicons::create_directories();
    let masterfile = pokemon::get_masterfile().await?;

    let circles = masterfile.get_pokemon_colors();
    let file_names = uicons::get_string_filenames("./pokemon-icons/_icons/SVG", false);

    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    for circle in circles {
        match image::create_svg(&circle) {
            Ok(_) => {}
            Err(err) => println!("Unable to save SVG for {} | {}", circle.id, err),
        }
        match image::create_png(&circle.get_filename(), "safe", 1., &fontdb) {
            Ok(_) => {}
            Err(err) => println!("Unable to save PNG for {} | {}", circle.id, err),
        }
        match image::create_webp(&circle.get_filename(), "safe") {
            Ok(_) => {}
            Err(err) => println!("Unable to save WebP for {} | {}", circle.id, err),
        }
    }

    let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for chunk in file_names.chunks((file_names.len() / cpus) + 1) {
        let pokemon = masterfile.pokemon.clone();
        let chunk = chunk.to_vec();
        let fontdb = fontdb.clone();
        let handle = thread::spawn(move || {
            for file in chunk {
                let new_name = pokemon::get_uicons_name(&file, &pokemon);
                match image::move_svg(&file, &new_name) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("Unable to move SVG for {} | {:}", new_name, err);
                        continue;
                    }
                }
                match image::create_png(&new_name, "artificial", 4., &fontdb) {
                    Ok(_) => {}
                    Err(_) => println!("Unable to create PNG for {}", new_name),
                }
                match image::create_webp(&new_name, "artificial") {
                    Ok(_) => {}
                    Err(_) => println!("Unable to create WEBP for {}", new_name),
                }
            }
        });
        handles.push(handle);
    }
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(_) => println!("Thread {} finished", i + 1),
            Err(_) => println!("Thread {} panicked", i + 1),
        }
    }

    uicons::create_jsons();
    println!("Finished in {}s", time.elapsed().as_secs_f32());
    Ok(())
}
