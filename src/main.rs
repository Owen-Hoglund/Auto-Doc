mod utility;
use utility::file_control;


fn main () {
    let project_path = r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc_test_directory\project_name".to_string();
    file_control::receptionist(project_path);
}

