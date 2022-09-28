
use crate::Encryptor;
use crate::my_get_user_input;

pub struct DataBase {
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
        return DataBase {
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
        self._website.push(lst[0].to_string());
        self._username.push(lst[1].to_string());
        self._password.push(lst[2].to_string());
        self._comment.push(lst[3].to_string());
    }

    pub fn print_data_base(self){
        for d in self._data {
            println!("{}", d.to_string());
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