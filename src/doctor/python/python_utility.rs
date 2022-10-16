use std::fs::File;
use std::io::Read;

pub fn file_splitter(path: &String) -> Vec<String> {
    // Opens the file that we want to create documentation for
    let mut example_file = File::open(path).expect("Can't Open File");

    // Feeds the entire file into a string
    let mut contents = String::new();
    example_file
        .read_to_string(&mut contents)
        .expect("Can't Read File");

    // Splits the massive code string into a vector of strings, each string comprises a line
    let x = contents
        .split("\r\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut result: Vec<String> = Vec::new();

    for line in x {
        if non_empty_line(&line) {
            result.push(line.to_string());
        }
    }
    result
}

fn non_empty_line(line: &String) -> bool {
    if line.chars().count() == 0 {
        return false;
    }
    for c in line.chars() {
        if c != ' ' {
            return true;
        }
    }
    false
}

pub fn spacing_comparison(line: &String, spaces: u8) -> bool {
    let mut space_count = 0;
    if line.chars().collect::<Vec<char>>()[0] == ')' {
        return true;
    }
    for c in line.chars() {
        if c == ' ' {
            space_count += 1;
        } else {
            break;
        }
    }

    if space_count > spaces {
        true
    } else {
        false
    }
}
pub fn determine(block: &Vec<String>) -> String {
    // let x = block[0].as_str();
    let x = block[0].split_whitespace().collect::<Vec<&str>>()[0];
    match x {
        "from" | "import" => "import".to_string(),
        "def" => "function".to_string(),
        "class" => "class".to_string(),
        _ => "variable".to_string(),
    }
}

pub fn is_comment(line: &String) -> bool {
    for c in line.chars() {
        match c {
            '#' => return true,
            ' ' => (),
            _ => return false,
        }
    }
    return false;
}

pub fn python_parser(content: &Vec<String>, content_type: &str, spacing: u8) -> Vec<Vec<String>> {
    let mut code_imports: Vec<Vec<String>> = Vec::new(); // This stores all import declarations in the file
    let mut code_functions: Vec<Vec<String>> = Vec::new(); // This stores all function definitions in the file
    let mut code_classes: Vec<Vec<String>> = Vec::new(); // This stores all class definitions in the file
    let mut code_variables: Vec<Vec<String>> = Vec::new(); // This stores all global variable definitions in the file
    let mut open_storage: Vec<String> = Vec::new(); // This temporarily stores code blocks for identification

    let mut lines = content.iter(); // Creates an iterator for us to loop through
    open_storage.push(lines.next().expect("Line is not empty?").to_string()); // Pushes the first line of the file into temporary storage

    for line in lines {
        if !is_comment(line) {
            if spacing_comparison(line, spacing) {
                open_storage.push(line.to_string());
            } else {
                let x = determine(&open_storage);
                let y = x.as_str();
                match y{
                     "import" => code_imports.push(open_storage.clone()),
                     "class" => code_classes.push(open_storage.clone()),
                     "function" => code_functions.push(open_storage.clone()),
                     "variable" => code_variables.push(open_storage.clone()),
                     _ => panic!("This should not be possible! world is falling! determine always produces 1 of 4 strings"),
                 }
                open_storage.clear();
                open_storage.push(line.to_string());
            }
        }
    }
    let x = determine(&open_storage);
    let y = x.as_str();
    match y{
                     "import" => code_imports.push(open_storage.clone()),
                     "class" => code_classes.push(open_storage.clone()),
                     "function" => code_functions.push(open_storage.clone()),
                     "variable" => code_variables.push(open_storage.clone()),
                     _ => panic!("This should not be possible! world is falling! determine always produces 1 of 4 strings"),
                 }
    open_storage.clear();
    //println!("\nContent Type: {}\n", content_type);
    match content_type{
          "import" => code_imports,
          "class" => code_classes,
          "function" => code_functions,
          "variable" => code_variables,
          _ => panic!("Unsupported parsing type. Supported types are 'import', 'class', 'function', 'variable'.")
     }
}

pub fn class_name(declaration: &String) -> String {
    declaration.split_whitespace().collect::<Vec<&str>>()[1]
        .chars()
        .filter(|x| x != &':')
        .collect::<Vec<char>>()
        .into_iter()
        .collect::<String>()
        .split("(")
        .collect::<Vec<&str>>()[0]
        .to_string()
}

pub fn function_name(declaration: &String) -> String {
    declaration.split("(").collect::<Vec<&str>>()[0]
        .replace("def ", "")
        .to_string()
}
