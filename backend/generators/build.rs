use shared::functions::{declare_modules, find_exported_struct, find_modules};
use std::{fs::File, io::Write};

fn main() {
    let generators = find_modules();
    let mut file = File::create("src/lib.rs").unwrap();
    declare_modules(&mut file, generators.clone());

    file = File::create("src/export.rs").unwrap();

    writeln!(
        &mut file,
        r#"use shared::traits::Generator;
use std::collections::HashMap;

pub fn get() -> HashMap<String, Box<dyn Generator>> {{
    let mut generators: HashMap<String, Box<dyn Generator>> = HashMap::new();
    
    let gs = ["#
    )
    .unwrap();

    let mut generators_iter = generators.iter().peekable();
    while let Some(generator) = generators_iter.next() {
        let module_file = File::open(format!("src/{}.rs", generator)).unwrap();
        let exported_struct = find_exported_struct(module_file).unwrap();

        writeln!(
            &mut file,
            r#"        crate::{}::{}"#,
            generator, exported_struct
        )
        .unwrap();

        if generators_iter.peek().is_none() {
            writeln!(&mut file, r#"    ];"#).unwrap();
        } else {
            writeln!(&mut file, ",").unwrap();
        }
    }

    writeln!(
        &mut file,
        r#"
    for g in gs {{
        generators.insert(g.name(), Box::new(g));
    }}

    generators
}}"#
    )
    .unwrap();
}
