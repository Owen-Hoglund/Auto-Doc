use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;

pub fn execute(imports: Vec<String>, guide_file: &String, project_folder: &String) -> HashMap<String, String>{
    let mut import_map: HashMap<String, String> = HashMap::new();
    populate_hashmap(&mut import_map, &imports, project_folder);
    write_imports(&import_map, &guide_file);
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

fn write_imports(import_map: &HashMap<String, String>, guide_file: &String){
    let mut guide = OpenOptions::new()
    .write(true)
    .append(true)
    .open(guide_file)
    .unwrap();

    if let Err(e) = writeln!(guide,
        "## Imports"
    ){eprintln!("Couldn't write to file: {}", e);}
    if import_map.is_empty(){
        if let Err(e) = write!(guide,
            "This file does not import from any modules or libaries that AutoDoc could detect"
        ){eprintln!("Couldn't write to file: {}", e);}
    } else{
        if let Err(e) = write!(guide,
            "-"
        ){eprintln!("Couldn't write to file: {}", e);}
        for mapping in import_map{
            if let Err(e) = write!(guide,
                "{}, ", mapping.1,
            ){eprintln!("Couldn't write to file: {}", e);}
        }
    }
    if let Err(e) = write!(guide,
        "\n"
    ){eprintln!("Couldn't write to file: {}", e);}
    
}
