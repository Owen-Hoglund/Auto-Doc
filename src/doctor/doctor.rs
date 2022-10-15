use crate::file_management;
use crate::doctor::python;

pub fn execute(guide_file: &String, original_file_path: &String, project_folder: &String){
    // Pointless for now but as more languages are brought on board this will match the files to their respective writers
    match  file_management::file_tools::language(&original_file_path){
        "Python" => python::python_documentation::execute(original_file_path, guide_file, project_folder),
        _=> panic!("Unsupported Language, consider helping us by implementing it :)")
    }
}