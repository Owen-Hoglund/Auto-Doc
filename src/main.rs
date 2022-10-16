mod file_management;
mod doctor;
use std::fs;
use std::time::Instant;
fn main() {
    let now = Instant::now();
    let project_path = r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc_test_directory\project_name".to_string();
    let destroyer = r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc_test_directory\project_name_Guide\project_name".to_string();
    fs::remove_dir_all(&destroyer).expect("This Directory Does not exist");
    file_management::file_control::execute(&project_path);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
