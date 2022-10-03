use crate::utility::*;

pub fn outer_parsing(content: Vec<String>){
    let mut code_global_variables: Vec<Vec<String>> = Vec::new(); // This stores all global variable definitions in the file
    let mut code_imports: Vec<Vec<String>> = Vec::new(); // This stores all import declarations in the file
    let mut code_functions: Vec<Vec<String>> = Vec::new(); // This stores all function definitions in the file
    let mut code_classes: Vec<Vec<String>> = Vec::new(); // This stores all class definitions in the file
    let mut open_storage: Vec<String> = Vec::new(); // This temporarily stores code blocks for identification 
    
    let mut lines = content.iter(); // Creates an iterator for us to loop through 
    open_storage.push(lines.next().expect("Line is not empty?").to_string()); // Pushes the first line of the file into temporary storage

    for line in lines{
        if !auto_doc_utility::is_comment(line){
            if auto_doc_utility::spacing_comparison(line, 0){
                open_storage.push(line.to_string());
            } else {
                let x = auto_doc_utility::determine(&open_storage);
                let y = x.as_str();
                match y{
                    "import" => code_imports.push(open_storage.clone()),
                    "class" => code_classes.push(open_storage.clone()),
                    "function" => code_functions.push(open_storage.clone()),
                    "variable" => code_global_variables.push(open_storage.clone()),
                    _ => panic!("This should not be possible! world is falling! determine always produces 1 of 4 strings"),
                }
                open_storage.clear();
                open_storage.push(line.to_string());
            }
        }
    }
    let x = auto_doc_utility::determine(&open_storage);
                let y = x.as_str();
                match y{
                    "import" => code_imports.push(open_storage.clone()),
                    "class" => code_classes.push(open_storage.clone()),
                    "function" => code_functions.push(open_storage.clone()),
                    "variable" => code_global_variables.push(open_storage.clone()),
                    _ => panic!("This should not be possible! world is falling! determine always produces 1 of 4 strings"),
                }
                open_storage.clear();

    //println!("{:?}", code_functions);
    write_imports::execute(code_imports);
    write_global_variables::execute(code_global_variables);
}


