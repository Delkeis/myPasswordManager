mod my_encryptor;
mod my_file_controller;
mod my_data_base;

use std::io;
use my_encryptor::encryptor::Encryptor;
use my_file_controller::file_controller::FileController;
use my_data_base::database::DataBase;

fn main() {

    let req = String::from("Password");
    let enc = Encryptor::new(my_get_user_input(req));

    println!("{}", enc.crypt_str(String::from("BITE")));

    let file_controller: FileController = FileController::new("./pass".to_string());
    let mut data_base: DataBase = DataBase::new();

    data_base.add_new_dataset(file_controller.unload_file());

    data_base.print_data_base();


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