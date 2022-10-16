use crate::doctor::python::import_mapping;
use crate::doctor::python::python_utility;
use crate::doctor::python::write_functions;
use crate::doctor::python::write_classes;
use std::collections::HashMap;
pub fn execute(original_file_path: &String, guide_file: &String, project_folder: &String) {
    // Splits Source Code by line for easier parsing
    let content = python_utility::file_splitter(original_file_path);

    // Parses the content for import declarations, then does a bit of work to make them easier to deal with later
    let temp_imports = python_utility::python_parser(&content, "import", 0);
    let imports = import_mapping::expanded_imports(temp_imports);
    let import_map: HashMap<String, String> =
        import_mapping::populate_hashmap(&imports, project_folder);

    // Parses the Content for
    let classes = python_utility::python_parser(&content, "class", 0);
    let functions = python_utility::python_parser(&content, "function", 0);

    write_classes::execute(&import_map, &classes, &guide_file);
    write_functions::execute(&import_map, &functions, &guide_file);

}
