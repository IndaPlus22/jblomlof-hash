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

    pub fn get_line(&self) -> String {
        let mut line = self.firstname.clone();
        line += ",";
        line += &self.surname;
        line += ",";
        line += &self.phonenumber;
        line
    }
}
