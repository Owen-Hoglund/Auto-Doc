use crate::doctor::python::python_utility;
use std::collections::HashMap;
use std::path::Path;
pub fn populate_hashmap(imports: &Vec<String>, project_folder: &String) -> HashMap<String, String> {
    // Creates a placeholder to return
    let mut import_map: HashMap<String, String> = HashMap::new();
    // Begin iterating over imports in the vector of imports
    for import in imports {
        // Some variables used for determining the import style
        let mut source: String = String::new();
        let mut specific: String = String::new();
        let mut alias: String = String::new();
        let mut builder = import.split_whitespace().peekable();
        // Traverse the import word by word
        while builder.peek().is_some() {
            // Checks if there is a next iteration to get
            match builder.next().unwrap() {
                "from" => {
                    // Checks if the import starts with "from", then appropriately assigns source and specific
                    source = builder.next().unwrap().to_string();
                    builder.next();
                    specific = builder.next().unwrap().to_string();
                }
                "import" => {
                    source = builder.next().unwrap().to_string();
                }
                "as" => {
                    alias = builder.next().unwrap().to_string();
                }
                _ => println!("Critical Error, Builder has reached unreachable state"),
            }
        }

        // Converts the python source to an actual path (import HamSandwich.IOUtils -> C::\Users\....\HamSandwich\IOUtils.py)
        let source_path = python_source_to_path(&source, project_folder);

        // Converts this to an 'real' Path so that we can check if that path exists locally.
        // This is crucial for determining whether or not the import is from an external or user defined source
        let temp = Path::new(&source_path);

        if temp.exists() {
            let mut key: String = String::new();
            if !alias.is_empty() {
                key = alias.clone();
                imports_from_file(&alias, &source_path, project_folder, &mut import_map);
            } else if !specific.is_empty() {
                key = specific.clone();
            } else {
                key = source.clone();
                imports_from_file(&alias, &source_path, project_folder, &mut import_map);
            }
            let value = internal_link_generator(
                &import_source_to_obsidian_path(&source),
                &alias,
                &specific,
            );
            import_map.insert(key, value.clone());
        }
    }
    import_map
}

fn imports_from_file(
    alias: &String,
    source_path: &String,
    project_folder: &String,
    import_map: &mut HashMap<String, String>,
) {
    let internal_link_path = source_path
        .replace(&[project_folder, r"\"].join("").to_string(), "")
        .replace(".py", "_guide")
        .replace(r"\", "/");
    println!("{}", internal_link_path);
    let content = python_utility::file_splitter(&source_path);
    let classes = python_utility::python_parser(&content, "class", 0);
    let functions = python_utility::python_parser(&content, "function", 0);

    for class in classes {
        let name = python_utility::class_name(&class[0]);
        let mut link = String::new();
        if alias.is_empty() {
            link = format_args!("[[{}#{}|{}]]", internal_link_path, &name, &name).to_string();
            import_map.insert(name.clone(), link);
        } else {
            link = format_args!("[[{}#{}|{}]]", internal_link_path, &name, &name).to_string();
            import_map.insert(alias.clone(), link);
        }
        println!("{}", name);
    }

    for function in functions {
        let name = python_utility::function_name(&function[0]);
        println!("Function Name {}", name);
        if alias.is_empty() {
            let link = format_args!("[[{}#{}|{}]]", internal_link_path, &name, &name).to_string();
            import_map.insert(name.clone(), link);
        } else {
            let link = format_args!("[[{}#{}|{}]]", internal_link_path, &name, alias).to_string();
            import_map.insert(alias.clone(), link);
        }
    }
}

pub fn expanded_imports(imports_section: Vec<Vec<String>>) -> Vec<String> {
    // Merges the import lines into 1d Vec with each import line comprising an element of the vector
    let mut temp_imports: Vec<String> = Vec::new();
    let mut imports: Vec<String> = Vec::new();
    for line in imports_section {
        temp_imports.push(line[0].clone());
    }
    for line in temp_imports {
        if line.contains(",") {
            for x in multi_import_splitter(line) {
                imports.push(x);
            }
        } else if line.contains("*") {
            imports.push(import_all_fixer(&line));
        } else {
            imports.push(line);
        }
    }
    imports
}

fn multi_import_splitter(import: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let y = &import
        .split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let iter = &y[3..];
    for i in iter {
        result.push(
            [
                "from ".to_string(),
                y[1].to_string(),
                " import ".to_string(),
                i.to_string(),
            ]
            .join(""),
        );
    }
    result
}

fn import_all_fixer(import: &String) -> String {
    let temp: Vec<String> = import
        .split_whitespace()
        .map(|x| x.to_string())
        .rev()
        .collect::<Vec<String>>();
    //println!("TEST 2: {:?}", temp);
    let x = temp.join(" ").replace("* ", "").replace(" from", "");
    //println!("{}", x);
    x
}

fn python_source_to_path(import: &String, project_folder: &String) -> String {
    let x: String = [
        project_folder.to_string(),
        import.replace(".", r"\").to_string(),
    ]
    .join(r"\");
    let y = [x, ".py".to_string()].join("");
    y
}

fn import_source_to_obsidian_path(import: &String) -> String {
    import.replace(".", "/")
}

fn internal_link_generator(path: &String, alias: &String, header: &String) -> String {
    let filename = path
        .split("/")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();
    if header.is_empty() {
        if alias.is_empty() {
            format!("[[{}_guide|{}]]", path, filename)
        } else {
            format!("[[{}_guide|{}]]", path, alias)
        }
    } else {
        if alias.is_empty() {
            format!("[[{}_guide#{}|{}]]", path, header, header)
        } else {
            format!("[[{}_guide#{}|{}]]", path, header, alias)
        }
    }
}
