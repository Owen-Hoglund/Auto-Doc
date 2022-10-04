use std::fs;
use std::path::Path;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;

pub fn receptionist(project_dir: String) {
    let parent_folder = get_parent_directory(&project_dir);
    let project_name = get_project_name(&project_dir);
    documentation_directory_creation(&parent_folder, &project_name).expect("Couldn't Create File");
    
    let all_files: Vec<String> = file_getter(&project_dir);
    mimic_directories(&all_files, &parent_folder, &project_name);
    
}

fn mimic_directories(files: &Vec<String>, parent_folder: &String, project_name: &String){
    for file in files {
        let new_file_directory: String = create_mimicked_path(&project_name, &parent_folder, &file);
        new_dir(&new_file_directory).expect("Could Not Create Directory");
        create_file_in_mimicked_directory(&new_file_directory, &file);
    }
}

fn create_mimicked_path(project_name: &String, parent_folder: &String, file_path: &String) -> String{
    let new_path_prefix: String = [parent_folder.to_string(), r"\".to_string(), project_name.to_string(), " Guide".to_string()].join("");
    let new_file_path_suffix = file_path.replace(parent_folder, "");
    let new_path_temp = [new_path_prefix, new_file_path_suffix].join(r"\");
    let new_path_final = new_path_temp.replace(r"\\", r"\");
    let file_name = filename(file_path);
    new_path_final.replace(&file_name, "")
}

fn file_getter(project_dir: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    file_extraction(project_dir.to_string(), &mut files);
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

fn create_file_in_mimicked_directory(file_path: &String, file: &String){
    let filename = filename(file);
    let path = [file_path.to_string(), filename.to_string()].join("").replace(".py", ".md");

    
    let file = OpenOptions::new().write(true)
                             .create_new(true)
                             .open(path);
   
    
    file.expect("Couldnt write to file").write_all(b"This will be in the file");
    
}

fn new_dir(path: &String) -> std::io::Result<()> {
    println!("{}", path);
    fs::create_dir_all(path)?;
    Ok(())
}

fn filename(file_path: &String) -> String{
    let filename = file_path.split(r"\").map(|x| x.to_string()).collect::<Vec<String>>().iter().rev().next().unwrap().to_string();
    filename
}
