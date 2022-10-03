use::std::fs::File;
use::std::io::Read;

pub fn file_splitter(path: String) -> Vec<String>{
     // Opens the file that we want to create documentation for
     //let path = Path::new(r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\auto_doc\test_files\dataBaseManager.py");
     let mut example_file = File::open(path).expect("Can't Open File");
     
     // Feeds the entire file into a string
     let mut contents = String::new();
     example_file.read_to_string(&mut contents).expect("Can't Read File");
     
     // Splits the massive code string into a vector of strings, each string comprises a line
     let x = contents.split("\r\n").map(|x| x.to_string()).collect::<Vec<String>>();

     let mut result: Vec<String> = Vec::new();

     for line in x{
          if non_empty_line(&line) {
               result.push(line.to_string());
          }
     }
     result
}


fn non_empty_line(line: &String) -> bool{
     if line.chars().count() == 0 {
          return false;
     }
     for c in line.chars() {
          if c != ' ' {
               return true;
          }
     }
     false
}

pub fn spacing_comparison(line: &String, spaces: usize) -> bool{
     let mut space_count = 0;
     if line.chars().collect::<Vec<char>>()[0] == ')' {
          return true;
     }
     for c in line.chars(){
          if c == ' '{
               space_count += 1;
          }
          else{
               break;
          }
     }
     
     if space_count > spaces {
          true
     } else {
          false
     }
}
pub fn determine(block: &Vec<String>) -> String{
     // let x = block[0].as_str();
     let x = block[0].split_whitespace().collect::<Vec<&str>>()[0];
     match x{
          "from" | "import" => "import".to_string(),
          "def" => "function".to_string(),
          "class" => "class".to_string(),
          _ => "variable".to_string()
     }
}

pub fn is_comment(line: &String) -> bool{
    for c in line.chars(){
          match c {
               '#' => return true,
               ' ' => (),
               _ => return false
          }
    }
    return false;
}