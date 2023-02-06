//To generate imaginary friends.
use names::{Generator, Name};

#[derive(Clone, PartialEq, Eq)]
pub struct Friend {
    pub firstname: String,
    pub surname: String,
    pub phonenumber: String,
}

impl Friend {
    pub fn from_line(line: String) -> Friend {
        let subsection: Vec<&str> = line.split(",").collect();
        if subsection.len() != 3 {
            panic!("Data is corrupted!");
        }
        Friend {
            firstname: subsection[0].to_string(),
            surname: subsection[1].to_string(),
            phonenumber: subsection[2].to_string(),
        }
    }

    pub fn new(first: &str, sur: &str, num: &str) -> Friend {
        Friend {
            firstname: first.to_string(),
            surname: sur.to_string(),
            phonenumber: num.to_string(),
        }
    }

    pub fn get_line(&self) -> String {
        let mut line = self.firstname.clone();
        line += ",";
        line += &self.surname;
        line += ",";
        line += &self.phonenumber;
        line
    }

    pub fn new_imaginary_friend() -> Friend {
        let mut gen = Generator::with_naming(Name::Numbered);
        let s = gen.next().unwrap();
        let subsection: Vec<&str> = s.split("-").collect();

        // it seems to be able to generate shit like "<black-and-white>-<noun>-<number>"
        if subsection.len() == 3 {
            Friend {
                firstname: subsection[0].to_string(),
                surname: subsection[1].to_string(),
                phonenumber: subsection[2].to_string(),
            }
        } else {
            Friend::new_imaginary_friend()
        }
    }
}
