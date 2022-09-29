use magic_crypt::MagicCryptTrait;

#[derive(Clone)]
pub struct Encryptor{
    master_password: String,
}

impl Encryptor{
    pub fn new(master_pass: String) -> Self {

        return Encryptor{
            master_password: master_pass
        };
    }

    pub fn crypt_str(self, str: String) -> String {
        let crpt = magic_crypt::new_magic_crypt!(self.master_password, 256);
        let crypted_str = crpt.encrypt_bytes_to_base64(str.as_bytes());

        return crypted_str;
    }
}