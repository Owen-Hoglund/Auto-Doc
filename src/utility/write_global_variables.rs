use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::collections::{HashMap};
pub fn execute(global_variables: Vec<Vec<String>>){
    let variables = name_definition(global_variables);
    write_global_variables(variables);
}

fn name_definition(var_defs: Vec<Vec<String>>) -> HashMap<String, String>{
    let mut defs:HashMap<String, String> = HashMap::new();
    for definition in var_defs{
        // Holding ONE Definition. Lets put this whole thing into a single string, REINSERT LINE BREAKS, then split on '='
        let def = definition.join("\n");
        let name_def:Vec<String> = def.split('=').map(|x| x.to_string()).collect::<Vec<String>>();
        let name = name_def[0].clone();
        let definition = name_def[1..].concat();
        defs.insert(name,definition);
    }
    defs
}

fn write_global_variables(variables: HashMap<String, String>){
    // This opens our variables section, to be included in the larger codefile doc for the current python file
    let mut variables_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc\test_files\global_variables_section.md")
        .unwrap();
    // This opens our definition reference file, where we will store 
        let mut reference_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc\test_files\quick_reference_definitions.md")
        .unwrap();
    
        for (key, value) in &variables {
        if let Err(e) = writeln!(variables_file, 
            "- [[quick_reference_definitions|{}]]", key
        ){eprintln!("Couldn't write to file: {}", e);}
        if let Err(e) = writeln!(reference_file, 
            "## {} \n {}\n\n", key, value
        ){eprintln!("Couldn't write to file: {}", e);}
    }
    
}