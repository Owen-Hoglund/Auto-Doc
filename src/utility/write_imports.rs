use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::collections::{HashMap};

pub fn execute(imports_section: Vec<Vec<String>>){
    let mut imports: Vec<String> = Vec::new();
    for line in imports_section{
        imports.push(line[0].clone());
    }

    // Returns the aliases for each import
    let  mut import_aliases = get_import_aliases(&imports);
    println!("{:?}", import_aliases);
    // Writes the imports section for the current file
    write_imports_section(imports, &mut import_aliases);
    //println!("{:?}", contents);
}

fn write_imports_section(imports: Vec<String>, aliases: &mut HashMap<String, String>){
    let mut example_new_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc\test_files\test_text.md")
        .unwrap();
    let builtins = module_descriptions();
    if let Err(e) = writeln!(example_new_file, 
        "# Imports"
    ){eprintln!("Couldn't write to file: {}", e);}

    for line in imports.iter(){
        let format = line.split(' ').collect::<Vec<&str>>();
        if builtins.contains_key(format[1]){
            match format[0] {
                "import" => if let Err(e) = writeln!(example_new_file, 
                    "`{}` - Where [[{}]] is '{}'", format.join(" "), format[1], builtins.get(format[1]).unwrap()
                )
                {eprintln!("Couldn't write to file: {}", e);},
    
                "from" => if let Err(e) = writeln!(example_new_file, 
                    "`{}` - Where [[{}]] is '{}', and {} is defined [[Filename @todo|here]]", format.join(" "),format[1], builtins.get(format[1]).unwrap(), format[3]
                )
                {eprintln!("Couldn't write to file: {}", e);},
                _ => panic!("Function: get_imports is Producing bad data"),
            }
        }else{
            match format[0] {
                "import" => if let Err(e) = writeln!(example_new_file, 
                    "`{}` - Where [[{}]] is user defined", format.join(" "), format[1]
                )
                {eprintln!("Couldn't write to file: {}", e);},
    
                "from" => if let Err(e) = writeln!(example_new_file, 
                    "`{}` - Where [[{}]] is user defined [[{}#{}|here]]", format.join(" "), format[1], format[1], format[3]
                )
                {eprintln!("Couldn't write to file: {}", e);},
                _ => panic!("Function: get_imports is Producing bad data"),
            }
        }
    }
    if let Err(e) = writeln!(example_new_file, 
        "## Aliases"
    ){eprintln!("Couldn't write to file: {}", e);}
    for (key, value) in &*aliases {
        if let Err(e) = writeln!(example_new_file, 
            "- [[{}]] as **{}**", value, key
            //"- [[{}]] as [[{}|{}]]", value, value, key
        ){eprintln!("Couldn't write to file: {}", e);}
    }

}

pub fn module_descriptions() -> HashMap<String, String>{
    let mut module_library = HashMap::new();

    let path = Path::new(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc\test_files\python_builtins.txt");
    let mut descriptions = File::open(path).expect("Can't Open File");
    let mut contents = String::new();

    descriptions.read_to_string(&mut contents).expect("Can't Read File");

    let lines = contents.split('\n').collect::<Vec<&str>>();

    for key_value in lines {
        let key = key_value.split_whitespace().collect::<Vec<&str>>()[0];
        let value: &str = &key_value.split_whitespace().collect::<Vec<&str>>()[1..].join(" ");
        //println!("{} - {}", key, value);
        module_library.insert(key.to_string(), value.to_string());
    }
    module_library
}


pub fn get_import_aliases(imp: &Vec<String>) -> HashMap<String, String>{
    let mut aliases: HashMap<String, String> = HashMap::new();
    let words = imp.join(" ").split_whitespace().map(|y| y.to_string()).collect::<Vec<String>>();
    for x in 0..words.len(){
        if words[x] == "as".to_string(){
            aliases.insert(words[x + 1].to_string(), words[x - 1].to_string());
        }
    }
    aliases
}