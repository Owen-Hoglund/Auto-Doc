mod file_management;
mod doctor;
fn main() {
    let project_path = r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc_test_directory\project_name".to_string();
    file_management::file_control::execute(&project_path);
}
