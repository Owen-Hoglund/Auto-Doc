mod utility;

use utility::auto_doc_utility as tools;
use utility::outer_parsing;
use utility::file_control;


fn main () {
    // let test_file: String = r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc\test_files\dataBaseManager.py".to_string();
    // let contents = tools::file_splitter(test_file);
    // outer_parsing::outer_parsing(contents);
    let project_path = r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc_test_directory\project_name".to_string();
    file_control::create_directory(project_path);
}

// const RESERVED_WORDS_PYTHON: [&str; 33] = ["False", "def", "if", "raise",
//                                         "None", "del", "import", "return",
//                                         "True", "elif", "in", "try",
//                                         "and", "else", "is", "while",
//                                         "as", "except", "lambda", "with",
//                                         "assert", "finally", "nonlocal", "yield",
//                                         "break", "for", "not",
//                                         "class", "from", "or",
//         