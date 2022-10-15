use crate::doctor::python::import_mapping;
use crate::doctor::python::write_functions;
use std::collections::HashMap;

pub fn execute(imports:Vec<String>, classes:Vec<Vec<String>>, functions:Vec<Vec<String>>, global_vars:Vec<Vec<String>>, guide_file: &String, project_folder: &String){
    let import_map:HashMap<String, String> = import_mapping::execute(&imports, &guide_file, project_folder);
    write_functions::execute(&import_map, &functions, &guide_file);
    //write_classes::execute(&import_map, &classes, &guide_file);
}