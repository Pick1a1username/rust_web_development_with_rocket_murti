pub trait Encryptable {
    fn encrypt(&self) -> String;
}

pub mod rot13;