#![allow(unused_variables, unused_imports, dead_code)]
use rand::Rng;

pub struct UserToken{
    url: String,
    token: String,
}
impl UserToken{
    pub fn new(&self, token_len: u8, url: String) -> UserToken {
        let mut rng = rand::thread_rng();
        let characters: &str = "abdefghijklmopqrstuvwxyzABDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
        let mut token = String::from("");
        for _i in 1..token_len {
            let pointer = rng.gen_range(0..characters.chars().count());
            token.push(characters.as_bytes()[pointer] as char);
        }
        return UserToken{url, token}
    }
}