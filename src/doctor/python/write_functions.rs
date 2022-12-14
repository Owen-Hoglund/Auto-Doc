use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn execute(imports: &HashMap<String, String>,
    functions: &Vec<Vec<String>>,
    guide_file: &String,) {
    
    let mut guide = OpenOptions::new()
        .write(true)
        .append(true)
        .open(guide_file)
        .unwrap();
    if let Err(e) = writeln!(guide, 
        "# Functions \n"
        ) {
        eprintln!("Couldn't write to file: {}", e);
    }

    for function in functions {
        write_function(function, guide_file, &imports);
    }
}

fn write_function(function: &Vec<String>, path: &String, imports: &HashMap<String, String>) {
    let mut guide = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    if let Err(e) = writeln!(guide,
        "---"
    ){eprintln!("Couldn't write to file: {}", e);}
    let title = function.iter().next().unwrap().to_string();
    if let Err(e) = writeln!(guide, "## {}\n", clean_title(title),) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let contents = function.join("\n");
    let mut import_links: Vec<String> = Vec::new();

    for mapping in imports {
        if contents.contains(mapping.0) {
            import_links.push(mapping.1.to_string());
        }
    }
    if import_links.is_empty() {
        if let Err(e) = writeln!(guide,
            "### Reliances\nThis Function has no local module/library reliances AutoDoc could detect"
        ){eprintln!("Couldn't write to file: {}", e);}
    } else {
        if let Err(e) = writeln!(guide,
            "### Reliances\nThis function relies on the following locally defined functions and classes"
        ){eprintln!("Couldn't write to file: {}", e);}
        if let Err(e) = write!(guide, "-") {
            eprintln!("Couldn't write to file: {}", e);
        }
        for link in import_links {
            if let Err(e) = write!(guide, "{}, ", link,) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
    if let Err(e) = write!(guide, "\n") {
        eprintln!("Couldn't write to file: {}", e);
    }
    if let Err(e) = writeln!(guide, "```python\n{}\n```", contents,) {
        eprintln!("Couldn't write to file: {}", e);
    }

}

fn clean_title(title: String) -> String {
    title
        .replace("def ", "")
        .split("(")
        .map(|x| x.to_string())
        .next()
        .unwrap()
}
