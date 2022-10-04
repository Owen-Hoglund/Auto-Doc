use std::fs;
use std::path::Path;

pub fn create_directory(project_dir: String) {
    let parent_folder = get_parent_directory(&project_dir);
    let project_name = get_project_name(&project_dir);
    documentation_directory_creation(&parent_folder, &project_name).expect("Couldn't Create File");
    let all_files: Vec<String> = file_getter(&project_dir);
}

fn file_getter(project_dir: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    file_extraction(project_dir.to_string(), &mut files);
    for file in &files {
        println!("{}", file);
    }
   files
}
fn file_extraction(current_dir: String, files: &mut Vec<String>){
    let directory = fs::read_dir(current_dir).unwrap();
    let elements = directory.map(|x| x.unwrap().path().to_str().expect("LOL BROOOOO WHAT").to_string());
    for path in elements{
        if is_file(&path){files.push(path)} else{file_extraction(path, files)}
    }
}

fn is_file(path: &String) -> bool{
    path.contains(".")
}

fn get_parent_directory(directory: &String) -> String{
    let mut dir = directory.split(r"\").map(|x| x.to_string()).collect::<Vec<String>>();
    dir.remove(dir.len() - 1);
    let result = dir.join(r"\");
    result
}

fn get_project_name(directory: &String) -> String {
    let dir: Vec<String> = directory.split(r"\").map(|x| x.to_string()).collect::<Vec<String>>();
    let x = &dir[dir.len() - 1];
    x.to_string()
}

fn documentation_directory_creation(path: &String, project_name: &String) -> std::io::Result<()> {
    
    let documentation_directory: String = [path.to_string(), r"\".to_string(), project_name.to_string(), " Guide".to_string()].join("");
    
    fs::create_dir_all(documentation_directory)?;
    Ok(())
}