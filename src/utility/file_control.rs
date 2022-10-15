use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use crate::utility::doctor;


pub fn receptionist(project_dir: String) {
    // Parent folder is the folder holding the directory of the project we want to create documentation for
    let parent_folder = get_parent_directory(&project_dir);

    // Project name is the name of the directory the project is kept in
    let project_name = get_project_name(&project_dir);

    // This creates the directory that we keep our documentation in. We intentionally keep it out of the source
    documentation_directory_creation(&parent_folder, &project_name).expect("Couldn't Create File");
    
    // This grabs every file in the directory by storing its complete path as a string
    let all_files: Vec<String> = file_getter(&project_dir);

    // This creates a structurally identical directory to the source directory within the documentation directory
    // It does not copy the files, but does create placeholder markdown files for all the codefile, storing path as metadata
    mimic_directories(&all_files, &parent_folder, &project_name, &project_dir);


}

// mimics directories of a file, replacing a parent folder with the documentation folder name 
fn mimic_directories(files: &Vec<String>, parent_folder: &String, project_name: &String, project_folder: &String){
    // loops through files in the vector
    for file in files {
        if is_python(file) & !is_init(file){
                // creates the new altered path (details at function)
            let new_file_directory: String = create_mimicked_path(&project_name, &parent_folder, &file);

            // Recursively creates the directory for the new path by making all the necessary parent folders
            new_dir(&new_file_directory).expect("Could Not Create Directory");

            // Places new markdown file corresponding to the original file in the new directory 
            create_file_in_mimicked_directory(&new_file_directory, &file, project_folder);
        }
    }
}

// Creates a mimicked path for directory creation
fn create_mimicked_path(project_name: &String, parent_folder: &String, file_path: &String) -> String{
    /*
    This takes:
    - a files full path, for example ...\Project_Folder\Project\1\2\3\4\file.py
    - the parent folder, for example ...\Project_Folder\
    - the projects name, for example "Project"

    1. It then concatenates the parent folder path with the project name + " Guide" 
        this creates a path like this: ... \Project_Folder\Project Guide.
    2. It then removes the parent folder path from the file path yielding \1\2\3\4\path.py
    3. it then concatenates the two to create the final path

    there is some bug happening here which duplicates backslashes, I couldnt figure out what was causing it
    so I simply do a replace("\\", "\") which does the trick but is inelegant. Would like to solve

    4. We then delete the filename from the end of the directory to avoid creation of a folder titled "codefile.py" */


    //1
    let new_path_prefix: String = [parent_folder.to_string(), r"\".to_string(), project_name.to_string(), "_Guide".to_string()].join("");

    //2
    let new_file_path_suffix = file_path.replace(parent_folder, "");

    //3
    let new_path_temp = [new_path_prefix, new_file_path_suffix].join(r"\");
    // Bug fix
    let new_path_final = new_path_temp.replace(r"\\", r"\");
    // 4
    let file_name = filename(file_path);
    // return
    new_path_final.replace(&file_name, "") 
}

// This function sets up the variables required to recursively retrieve every file in the directory
fn file_getter(project_dir: &String) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    // Begin recursion
    file_extraction(project_dir.to_string(), &mut files);
    // Return Files
    files
}

// Grabs all the elements in a directory (folders and files), adds the files to the files vec, and calls itself upon the folders
fn file_extraction(current_dir: String, files: &mut Vec<String>){
    // get all elements
    let directory = fs::read_dir(current_dir).unwrap();

    // converts the elements into usable string form by unwrapping the elements into a path, then converting those to a string
    let elements = directory.map(|x| x.unwrap().path().to_str().expect("Couldnt convert path in direntry").to_string());
    // Loops through the paths, checks if the path represents a file or a folder, adds or recurs
    for path in elements{
        if is_file(&path){files.push(path)} else{file_extraction(path, files)}
    }
}

// Primitive method of checking if a path represents a file. Very prone to error if a user has '.' in any folder names
fn is_file(path: &String) -> bool{
    path.contains(".")
}
fn is_python(path: &String) -> bool{
    path.contains(".py")
}

fn is_init(path: &String) -> bool{
    path.contains("__init__")
}

// Returns the path of the folder one level above the directory passed in
fn get_parent_directory(directory: &String) -> String{
    let mut dir = directory.split(r"\").map(|x| x.to_string()).collect::<Vec<String>>();
    dir.remove(dir.len() - 1);
    let result = dir.join(r"\");
    result
}

// Grabs the name of the directory passed in. I think this might be functionally identical to get_file ? Check on this
fn get_project_name(directory: &String) -> String {
    let dir: Vec<String> = directory.split(r"\").map(|x| x.to_string()).collect::<Vec<String>>();
    let x = &dir[dir.len() - 1];
    x.to_string()
}

// This creates the folder that will hold all of our documentation for a given project
// It stores it in the same directory as the project and titles it "Project123 Guide"
fn documentation_directory_creation(path: &String, project_name: &String) -> std::io::Result<()> {
    let documentation_directory: String = [path.to_string(), r"\".to_string(), project_name.to_string(), "_Guide".to_string()].join("");
    fs::create_dir_all(documentation_directory)?;
    Ok(())
}

// Takes a path and a path for that file and creates a markdown file in that place 
fn create_file_in_mimicked_directory(file_path: &String, original_file: &String, project_folder: &String){
    let filename = filename(original_file);
    let path = [file_path.to_string(), filename.to_string()].join("")
                            .replace(".py", "_guide.md");
    {
        let mut file = OpenOptions::new()
                                .write(true)
                                .append(true)
                                .create_new(true)
                                .open(&path)
                                .unwrap();
        if let Err(e) = writeln!(file, 
            "# {}", filename
        ){eprintln!("Couldn't write to file: {}", e);}
    }
    doctor::doctor(&path, original_file, project_folder);
}

// Creates a new directory for a given path
fn new_dir(path: &String) -> std::io::Result<()> {
    //println!("{}", path);
    fs::create_dir_all(path)?;
    Ok(())
}

// Returns File name at
fn filename(file_path: &String) -> String{
    let filename = file_path.split(r"\").map(|x| x.to_string()).collect::<Vec<String>>().iter().rev().next().unwrap().to_string();
    filename
}
