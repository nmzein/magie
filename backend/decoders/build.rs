use shared::functions::{declare_modules, find_exported_struct, find_modules};
use std::{fs::File, io::Write};

fn main() {
    let decoders = find_modules();
    let mut file = File::create("src/lib.rs").unwrap();
    declare_modules(&mut file, decoders.clone());

    file = File::create("src/export.rs").unwrap();

    writeln!(
        &mut file,
        r#"use shared::traits::Decoder;
        
pub fn get() -> Vec<Box<dyn Decoder>> {{"#
    )
    .unwrap();

    if decoders.is_empty() {
        writeln!(
            &mut file,
            r#"
    vec![]
}}
"#
        )
        .unwrap();
        return;
    }

    writeln!(&mut file, r#"    vec!["#).unwrap();

    let mut decoders_iter = decoders.iter().peekable();
    while let Some(decoder) = decoders_iter.next() {
        let module_file = File::open(format!("src/{}.rs", decoder)).unwrap();
        let exported_struct = find_exported_struct(module_file).unwrap();

        writeln!(
            &mut file,
            r#"        Box::new(crate::{}::{})"#,
            decoder, exported_struct
        )
        .unwrap();

        if decoders_iter.peek().is_none() {
            writeln!(&mut file, r#"    ]"#).unwrap();
        } else {
            writeln!(&mut file, ",").unwrap();
        }
    }

    writeln!(&mut file, r#"}}"#).unwrap();
}
