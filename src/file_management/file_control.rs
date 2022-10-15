use crate::file_management::file_tools;
use std::fs;

pub fn execute(project_directory: &String) {
    // gets the parent directory of the project directory so that our guide directory does not go into the same directory as the source code
    let parent_folder = file_tools::get_parent_directory(project_directory);

    // Project name is the name of the directory the project is kept in
    let project_name = file_tools::get_current_directory_name(project_directory);

    // This creates the directory that we keep our documentation in. We intentionally keep it out of the source
    documentation_directory_creation(&parent_folder, &project_name).expect("Couldn't Create File");

    // This grabs every file in the directory by storing its complete path as a string
    let all_files: Vec<String> = file_tools::file_getter(&project_directory);

    // This creates a structurally identical directory to the source directory within the documentation directory
    // It does not copy the files, but does create placeholder markdown files for all the codefile, storing path as metadata
    file_tools::mimic_directories(
        &all_files,
        &parent_folder,
        &project_name,
        &project_directory,
    );
}

fn documentation_directory_creation(path: &String, project_name: &String) -> std::io::Result<()> {
    let documentation_directory: String = [
        path.to_string(),
        r"\".to_string(),
        project_name.to_string(),
        "_Guide".to_string(),
    ]
    .join("");
    fs::create_dir_all(documentation_directory)?;
    Ok(())
}
