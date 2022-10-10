use crate::utility::outer_parsing;


pub fn doctor(guide_file_path: &String, original_file_path: &String, project_folder: &String){
    println!("-------------------------------\nExecuting file creation \nSource File: {}\nFile guide {} \n \n", original_file_path, guide_file_path);
    outer_parsing::outer_parsing(original_file_path, guide_file_path, project_folder);
}