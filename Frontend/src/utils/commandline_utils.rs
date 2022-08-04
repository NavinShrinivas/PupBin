use std::env;
use crate::actions::help;

pub fn commandline_processer() -> crate::CommandLineData {
    let mut ret_struct = crate::CommandLineData::new();

    for (i, value) in std::env::args().into_iter().enumerate() {
        if &value == "--paste" {
            let file_path = match env::args().nth(i + 1) {
                Some(path) => path,
                None => {
                    panic!("Please provide a file to paste content")
                }
            };
            ret_struct.file_path = file_path;
            ret_struct.paste = true;
            ret_struct.get = false;
        }
        if &value == "--time" {
            let string_lifetime: String = match env::args().nth(i + 1) {
                Some(string_lifetime) => {
                    let mut temp_string = String::new();
                    if string_lifetime.len() == 5 {
                        let vec_lifetime: Vec<char> = string_lifetime.chars().collect();
                        if vec_lifetime[2] == ':' {
                            for (_, value) in vec_lifetime.into_iter().enumerate() {
                                if value != ':' {
                                    temp_string = format!("{}{}", temp_string, value);
                                }
                            }
                        } else {
                            panic!("Please give lifetime in correct format and correct encoding (HH:MM)")
                        }
                    } else {
                        panic!(
                            "Please give lifetime in correct format and correct encoding (HH:MM)"
                            )
                    }
                    temp_string
                }
                _ => {
                    String::from("0030") //Default 30 mins lifetime
                }
            };
            println!("{}", string_lifetime);
            ret_struct.lifetime = string_lifetime;
        }
        if &value == "--get" {
            let get_key = match env::args().nth(i + 1) {
                Some(link) => link,
                None => {
                    panic!("Please provide a link to get paste content")
                }
            };
            ret_struct.get_key = get_key;
            ret_struct.paste = false;
            ret_struct.get = true;
        }
        if &value == "--verbose" {
            ret_struct.verbose = true;
        }
        if &value == "--help"{
            help::help_print();
        }
    }
    return ret_struct;
}
