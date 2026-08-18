//! CLI entrypoint: `aegilex-core-bindgen <wit-dir> <world-name> <out.rs>`.

use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next();
    match command.as_deref() {
        Some("collect") => {
            let wit_dir = args
                .next()
                .expect("usage: aegilex-core-bindgen collect <wit-dir> <world-name>");
            let world_name = args.next().expect("usage: collect <wit-dir> <world-name>");
            let (resolve, world) =
                aegilex_core_bindgen::parse_world(Path::new(&wit_dir), &world_name)
                    .unwrap_or_else(|error| panic!("{error}"));
            let (counts, functions) =
                aegilex_core_bindgen::collect_world_import_instructions(&resolve, world);
            println!("functions: {}", functions.len());
            for (name, count) in counts {
                println!("{name}: {count}");
            }
        }
        _ => {
            let wit_dir = command
                .expect("usage: aegilex-core-bindgen <wit-dir> <world-name> <out.rs> | collect <wit-dir> <world-name>");
            let world_name = args
                .next()
                .expect("usage: aegilex-core-bindgen <wit-dir> <world-name> <out.rs>");
            let out = args
                .next()
                .expect("usage: aegilex-core-bindgen <wit-dir> <world-name> <out.rs>");

            let (resolve, world) =
                aegilex_core_bindgen::parse_world(Path::new(&wit_dir), &world_name)
                    .unwrap_or_else(|error| panic!("{error}"));
            let mut bindgen = aegilex_core_bindgen::Bindgen::new(resolve, world);
            std::fs::write(&out, bindgen.generate()).expect("failed to write generated bindings");
            println!("wrote {}", out);
        }
    }
}
