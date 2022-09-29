use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
// use std::env;
use std::collections::{HashMap};

// const RESERVED_WORDS_PYTHON: [&str; 33] = ["False", "def", "if", "raise",
//                                         "None", "del", "import", "return",
//                                         "True", "elif", "in", "try",
//                                         "and", "else", "is", "while",
//                                         "as", "except", "lambda", "with",
//                                         "assert", "finally", "nonlocal", "yield",
//                                         "break", "for", "not",
//                                         "class", "from", "or",
//                                         "continue", "global", "pass"];
fn main () {
    //env::set_var("RUST_BACKTRACE", "1");
    // Opens the file that we want to create documentation for
    let path = Path::new(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc\test_files\dataBaseManager.py");
    let mut example_file = File::open(path).expect("Can't Open File");
    
    // Feeds the entire file into a string
    let mut contents = String::new();
    example_file.read_to_string(&mut contents).expect("Can't Read File");
    
    // Splits the massive code string into a vector of strings, each string comprises a line
    let lines = contents.split("\r\n").map(|x| x.to_string()).collect::<Vec<String>>();
    
    // This Returns only the lines that include an import
    let imports = get_imports(lines);
    //println!("{:?}", imports);
    write_imports_section(imports);
}

fn get_imports(code_file: Vec<String>) -> Vec<String>{
    let mut imports: Vec<String> = Vec::new();
    for line in code_file.iter(){
        if line.split_whitespace().count() != 0{
            //println!("{}", line);
            let first_word = line.split_whitespace().collect::<Vec<&str>>()[0];
            match first_word {
                "import" => imports.push(line.to_string()),
                "from" => imports.push(line.to_string()),
                _ => (),
            }
        }
    }
    imports
}

fn write_imports_section(imports: Vec<String>){
    let mut example_new_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc\test_files\test_text.md")
        .unwrap();
    let builtins = module_descriptions();
    if let Err(e) = writeln!(example_new_file, 
        "# Imports and Import Aliases"
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
}

fn module_descriptions() -> HashMap<String, String>{
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