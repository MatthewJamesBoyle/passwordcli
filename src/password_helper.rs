use std::io::{Error, ErrorKind};

pub struct Service {}

impl Service {
    pub fn get_chars(&mut self, password: String, chars_needed: Vec<usize>) -> Result<String, Error> {
        if password.len() == 0 { Err(Error::new(ErrorKind::InvalidData, "password cannot be empty")) } else {
            return match chars_needed.len() {
                0 => Err(Error::new(ErrorKind::InvalidData, "no chars requested")),

                _ => {
                    let pass_as_vec: Vec<char> = password.chars().collect();
                    let mut res = String::new();

                    for (i, c) in chars_needed.iter().enumerate() {
                        // we need to - 1 off of the index as people ask for the "1st character, not the 0th"
                        if *c > pass_as_vec.len() - 1 || *c == 0 {
                            return Err(Error::new(ErrorKind::InvalidData, "You requested an index out of bounds"));
                        }
                        res.push(pass_as_vec[*c - 1]);
                        if i != chars_needed.len() - 1 {
                            res.push_str(",")
                        }
                    }

                    Ok(res)
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chars_for_1_index() -> Result<(), Error> {
        let mut svc = Service {};
        let mypass = "test";
        let chars_of_interest = vec![1];
        assert_eq!(svc.get_chars(mypass.to_string(), chars_of_interest)?, "t".to_string());
        Ok(())
    }

    #[test]
    fn test_get_chars_for_5_index() -> Result<(), Error> {
        let mut svc = Service {};
        let mypass = "testtest";
        let chars_of_interest = vec![1, 3, 5];
        assert_eq!(svc.get_chars(mypass.to_string(), chars_of_interest)?, "t,s,t".to_string());
        Ok(())
    }

    #[test]
    fn test_get_chars_handles_error_of_empty_password() {
        let mut svc = Service {};
        let mypass = "";
        let chars_of_interest = vec![1, 3, 5];
        match svc.get_chars(mypass.to_string(), chars_of_interest) {
            Ok(_) => assert!(false, "this should have errored"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_get_chars_handles_error_of_empty_chars_looking_for() {
        let mut svc = Service {};
        let mypass = "legit_password";
        let chars_of_interest = vec![];
        match svc.get_chars(mypass.to_string(), chars_of_interest) {
            Ok(_) => assert!(false, "this should have errored"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn test_get_chars_handles_error_if_i_pass_an_out_of_bound_char() {
        let mut svc = Service {};
        let mypass = "legit_password";
        let chars_of_interest = vec![99];
        match svc.get_chars(mypass.to_string(), chars_of_interest) {
            Ok(_) => assert!(false, "this should have errored"),
            Err(_) => assert!(true)
        }
    }
}
