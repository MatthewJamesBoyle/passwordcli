mod password_helper;

fn main() {
    let password = std::env::args().nth(1).expect("no password given");
    let chars_needed = std::env::args().nth(2).expect("no chars given");

    let mut chars_of_interest = vec![];
    let chars_as_vec: Vec<&str> = chars_needed.split(",").collect();
    for c in chars_as_vec {
        let a = c.parse().expect("chars must be a comma deliminated list such as 1,3,5");
        chars_of_interest.push(a)
    }

    let mut svc = password_helper::Service {};

    let res = svc.get_chars(password, chars_of_interest);
    match res {
        Ok(val) => println!("{}", val),
        Err(err) => println!("failed for some reason, {}", err)
    }
}

