use rand::{thread_rng, Rng};

use crate::psgen_args::{NoValue, PasswordStrength};


pub struct Password{
    charset:Vec<char>,
    length:u32,
}

impl Password {
    pub fn new(length:Option<u32>,password_level:PasswordStrength,no_value:NoValue)->Self{
        let pass_length = match length {
            Some(len) => len,
            None => password_level.get_pass_length_from_level(),
        };

        match no_value {
            NoValue::None => {
                Self{
                    charset:"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect(),
                    length:pass_length,
                }
            },
            NoValue::Symbol => {
                Self{
                    charset:"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect(),
                    length:pass_length,
                }
            },
            NoValue::Number => {
                Self{
                    charset:"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect(),
                    length:pass_length,
                }
            },
            NoValue::NumberAndSymbol => {
                Self{
                    charset:"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars().collect(),
                    length:pass_length,
                }
            },
        }
    }

    pub fn generate(&self)->String{
        let mut rng = thread_rng();
        let password:String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..self.charset.len());
                self.charset[idx]
            })
            .collect();

        password
    }
}