mod modules{
    pub mod database;
    pub mod file_controller;
    pub mod encryptor;
}

use std::io;
use modules::encryptor::Encryptor;
use modules::file_controller::{FileController, TraitFile};
use modules::database::DataBase;

fn main() {
    // creation database controller et le path du fichier de sauvegarde
    let path: String = String::from("./pass");
    let mut data_base: DataBase = DataBase::new();

    //création d'un file controller pour récupérer le contenue du fichier de sauvegarde 
    let file_controller: FileController = FileController::new(path.clone());
    data_base.add_new_dataset(file_controller.unload_file());

    //debug 
    data_base.clone().print_loud_data_base();

    // création d'un file controller pour écrire dans le fichier de sauvegarde
    let file_controller = FileController::new(path);
    file_controller.load_file_from_string(data_base.clone().unload_data_in_string());
}


//////////////////////////////////////////////////
/// 
/// Outils généraux 
/// 
//////////////////////////////////////////////////

fn my_get_user_input(request: String) -> String {
    let mut str = String::new();
    println!("Please input {} :", request);

    match io::stdin().read_line(&mut str){
        Err(why) => {
            println!("Error : {}", why);
            return str;
        },
        Ok(_) => return str
    }
}