mod modules{
    pub mod database;
    pub mod file_controller;
    pub mod encryptor;
}

use std::io;
use modules::encryptor::Encryptor;
use modules::file_controller::FileController;
use modules::database::DataBase;

fn main() {

    let file_controller: FileController = FileController::new("./pass".to_string());
    let mut data_base: DataBase = DataBase::new();

    data_base.add_new_dataset(file_controller.unload_file());

    //data_base.clone().print_data_base();

    data_base.print_loud_data_base();
}


//////////////////////////////////////////////////
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