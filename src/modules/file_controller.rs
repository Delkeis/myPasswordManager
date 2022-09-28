use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;

pub struct FileController{
    _file: File,
}

impl FileController{

    // Constructor
    pub fn new(path_str: String) -> Self {

        let path = Path::new(&path_str);
        let mut _file_desc: File = match File::open(path){
            Ok(_file_desc) => return FileController{_file: _file_desc},
            Err(why) => {
                println!("Error : {}", why);

                match File::create(&path_str) {
                    Ok(_) => return FileController::new(path_str),
                    Err(why) => {
                        panic!("Error {} !", why);
                    }
                };
            },
        };
    }


    // on décharge le contenu du fichier et on retourne une liste
    // de type Vector<String>
    pub fn unload_file(self) -> Vec<String> {
        let mut tab: Vec<String> = Vec::new();
        let mut buffer = BufReader::new(self._file);
        let mut s: String = String::new();

        loop{
            //len(ou la variable dans Ok()) est une variable temporraire 
            // qui récupère la valeur de retour de match
            match buffer.read_line(&mut s) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    else{
                        tab.push(s.replace("\n,", ""));
                        continue;
                    }
                },
                Err(why) => {
                    println!("Error : {}", why);
                    break;
                }
            }
        }
        return tab;
    }
}