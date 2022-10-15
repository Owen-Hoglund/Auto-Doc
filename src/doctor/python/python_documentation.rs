use crate::doctor::python::python_utility;
pub fn execute(original_file_path: &String, guide_file: &String, project_folder: &String){
    python_utility::parse_file(original_file_path, guide_file, project_folder);
}
