use rand::{thread_rng, Rng};

fn main() {
    let mut mina: Vec<char> = vec![];
    loop {
        let mut foo = thread_rng();
        let bagh = foo.gen::<char>();
        //Option for choosing between a..z 0..9 and special characters
        //to be added later
        if bagh.is_ascii() && !bagh.is_ascii_control() {
            /*println!("yes the number is {}", bagh);*/
            mina.push(bagh);
            //The length of your password
            if mina.len() == 18 {
                /*println!("{:?}", mina);*/
                let sina: Vec<String> = mina.iter().map(|x| x.to_string()).collect();
                println!("{}", sina.join(""));
                break;
            }
        }
    }
}
