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

    pub fn contents(&self) -> String {
        match std::fs::read_to_string(self.input.clone()) {
            Ok(file) => match (self.nums, self.reverse) {
                (true, true) => file
                    .split('\n')
                    .enumerate()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join("\n")
                    .split('\n')
                    .rev()
                    .take(self.lines + 1)
                    .collect::<Vec<&str>>()
                    .join("\n")
                    .split('\n')
                    .rev()
                    .collect::<Vec<&str>>()
                    .join("\n"),
                (_, true) => file
                    .split('\n')
                    .rev()
                    .take(self.lines + 1)
                    .collect::<Vec<&str>>()
                    .join("\n")
                    .split('\n')
                    .rev()
                    .collect::<Vec<&str>>()
                    .join("\n"),
                (true, _) => file
                    .split('\n')
                    .enumerate()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join("\n")
                    .split('\n')
                    .take(self.lines + 1)
                    .collect::<Vec<&str>>()
                    .join("\n"),
                (_, _) => file
                    .split('\n')
                    .take(self.lines)
                    .collect::<Vec<&str>>()
                    .join("\n"),
            },
            Err(e) => {
                println!("{}", e);
                exit(1)
            }
        }
    }
}
