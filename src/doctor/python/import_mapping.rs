use std::collections::HashMap;
use std::path::Path;
use crate::doctor::python::python_utility;
use std::fs::OpenOptions;
use std::io::prelude::*;

use super::python_utility::file_splitter;
pub fn execute(imports: &Vec<String>, guide_file: &String, project_folder: &String) -> HashMap<String, String>{
    let mut import_map: HashMap<String, String> = HashMap::new();
    populate_hashmap(&mut import_map, &imports, project_folder);
    //write_imports(&import_map, &guide_file);
    import_map
}

fn populate_hashmap(import_map: &mut HashMap<String,String>, imports: &Vec<String>, project_folder: &String){
    for import in imports{
        let mut source: String = String::new();
        let mut specific: String = String::new();
        let mut alias: String = String::new();
        let mut builder = import.split_whitespace().peekable();
        // Traverse the import word by word
        while builder.peek().is_some() { // Checks if there is a next iteration to get
            match builder.next().unwrap() {
                "from" => { // Checks if the import starts with "from", then appropriately assigns source and specific
                    source = builder.next().unwrap().to_string();
                    builder.next();
                    specific = builder.next().unwrap().to_string();
                },
                "import" => {
                    source = builder.next().unwrap().to_string();
                },
                "as" => {
                    alias = builder.next().unwrap().to_string();
                },
                _ => println!("Critical Error, Builder has reached unreachable state"),
            }
        }

        let temp = python_source_to_path(&source, project_folder);
        let source_path= Path::new(&temp);
        if source_path.exists() {
            let mut key:String = String::new();
            if alias.is_empty(){
                key = source.clone();
            } else{
                key = alias.clone();
            }
            let value = internal_link_generator(&import_source_to_obsidian_path(&source), &alias, &specific);
            import_map.insert(key, value);
            imports_from_file(source_path, import_map, alias);
        }
    }
}


fn python_source_to_path(import: &String, project_folder: &String) -> String{
    let x: String = [project_folder.to_string(), import.replace(".", r"\").to_string()].join(r"\");
    let y = [x, ".py".to_string()].join("");
    y
}

fn import_source_to_obsidian_path(import: &String) -> String{
    import.replace(".", "/")
}

fn internal_link_generator(path: &String, alias: &String, header: &String) -> String{
    if header.is_empty(){
        if alias.is_empty(){
            format!("[[{}_guide]]", path)
        }
        else {
            format!("[[{}_guide|{}]]", path, alias)
        }
    }
    else {
        if alias.is_empty(){
            format!("[[{}_guide#{}]]", path, header)
        }
        else {
            format!("[[{}_guide#{}|{}]]", path, header, alias)
        }
    }
}
fn imports_from_file(source_path: &Path, import_map: &mut HashMap<String,String>, alias: String){
    let content = file_splitter(&source_path.to_str().unwrap().to_string());

}