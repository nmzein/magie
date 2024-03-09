use shared::traits::Generator;
use std::collections::HashMap;

pub fn get() -> HashMap<String, Box<dyn Generator>> {
    let mut generators: HashMap<String, Box<dyn Generator>> = HashMap::new();
    
    let gs = [
        crate::tiatoolbox::TIAToolbox
    ];

    for g in gs {
        generators.insert(g.name(), Box::new(g));
    }

    generators
}
