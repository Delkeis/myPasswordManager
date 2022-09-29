
use crate::Encryptor;
use crate::my_get_user_input;

#[derive(Clone)]
pub struct DataBase {
    _enc: Encryptor,
    _data: Vec<String>,

    _website: Vec<String>,
    _username: Vec<String>,
    _password: Vec<String>,
    _comment: Vec<String>
}

impl DataBase{

    ///////////////////////////////////////////////////////////////
    /// 
    ///                     PUBLIC
    ///
    ///////////////////////////////////////////////////////////////

    // Constructor
    pub fn new() -> Self {
        let req: String = my_get_user_input("Password".to_string());

        return DataBase {
            _enc: Encryptor::new(req),
            _data: Vec::new(), 
            _website: Vec::new(),
            _username: Vec::new(),
            _password: Vec::new(),
            _comment: Vec::new()
        };
    }

    pub fn add_new_dataset(&mut self, datas: Vec<String>){
        self._data = Vec::new();
        for d in datas {
            self._data.push(d);
        }
        self.unload_dataset();
    }

    // on parse la ligne pour la push dans  les tableaux exploitables
    // par la suite 
    pub fn add_new_data(&mut self, data_line: String){
        let lst: Vec<&str> = data_line.as_str().split(" ").collect();

        let str: String = self._enc.clone().crypt_str(lst[0].to_string());
        self._website.push(str);
        
        let str: String = self._enc.clone().crypt_str(lst[1].to_string());
        self._username.push(str);

        let str: String = self._enc.clone().crypt_str(lst[2].to_string());
        self._password.push(str);

        let str: String = self._enc.clone().crypt_str(lst[3].to_string());
        self._comment.push(str);
        }

    pub fn _print_data_base(self){
        for d in self._data {
            println!("{}", d.to_string());
        }
    }

    pub fn print_loud_data_base(self){
        let mut i = 0;
        while i < self._website.len(){
            println!("website : {}, username: {}, password, {}, comment: {}", 
            self._website[i],
            self._username[i],
            self._password[i],
            self._comment[i]);

            i += 1;
        }
    }

    ///////////////////////////////////////////////////////////////
    /// 
    ///                     PRIVATE
    ///
    ///////////////////////////////////////////////////////////////

    /// On récuipère le contenu du dataset pour les push dans 
    /// les différentes variables
    fn unload_dataset(&mut self){
        let mut data: Vec<String> = Vec::new();

        for d in &self._data { data.push(d.to_string());}
        for dat in &data {
            self.add_new_data(dat.to_string());
        }
    }

}