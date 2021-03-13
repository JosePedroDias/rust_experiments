use std::env;

#[derive(Debug)]
pub struct CmdLineParams {
    pub num_pieces: usize,
    pub full_screen: bool,
}

impl Default for CmdLineParams {
    fn default() -> Self {
        Self {
            num_pieces: 28,
            full_screen: false,
        }
    }
}

pub fn parse_arguments() -> CmdLineParams {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut params: CmdLineParams = CmdLineParams {
        ..Default::default()
    };

    for arg in args {
        if arg.starts_with("--pieces=") {
            let v = &arg[9..];
            println!("{}", v);
            params.num_pieces = usize::from_str_radix(v, 10).unwrap();
        } else if arg.starts_with("--fs") {
            params.full_screen = true
        }
    }

    params
}
