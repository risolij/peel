use std::fs;
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Peel {
    #[structopt(short, long)]
    input: String,

    #[structopt(short, long)]
    reverse: bool,

    #[structopt(short, long, default_value = "10")]
    lines: usize,

    #[structopt(short, long)]
    nums: bool,

    #[structopt(short, long)]
    verbose: bool,
}

impl Peel {
    pub fn print_header(self) -> Self {
        if self.verbose == true {
            println!("----> {} <----", self.input);
        };

        self
    }

    pub fn enumerate_file(&self, file: String) -> String {
        let mut file = file
            .split('\n')
            .enumerate()
            .map(|(k, v)| format!("{}: {}", k  + 1, v))
            .collect::<Vec<String>>();

        file.pop();
        file.join("\n")
    }

    pub fn reverse(&self, file: String) -> String {
        file.split('\n')
            .rev()
            .take(self.lines)
            .collect::<Vec<&str>>()
            .join("\n")
            .split('\n')
            .rev()
            .collect::<Vec<&str>>()
            .join("\n")
    }

    pub fn normal(&self, file: String) -> String {
        file.split('\n')
            .take(self.lines)
            .collect::<Vec<&str>>()
            .join("\n")
    }

    pub fn contents(&self) -> String {
        match fs::read_to_string(self.input.clone()) {
            Ok(file) => match (self.nums, self.reverse) {
                (true, true) => self.reverse(self.enumerate_file(file)),
                (_, true) => self.reverse(file),
                (true, _) => self.normal(self.enumerate_file(file)),
                (_, _) => self.normal(file),
            },
            Err(e) => {
                println!("{}", e);
                exit(1)
            }
        }
    }
}
