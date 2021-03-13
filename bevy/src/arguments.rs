use bevy::math::Vec2;
use std::env;
use std::process;

#[derive(Debug)]
pub struct CmdLineParams {
    pub num_pieces: usize,
    pub full_screen: bool,
    pub image_index: Option<usize>,
    pub screen_dims: Vec2,
}

impl Default for CmdLineParams {
    fn default() -> Self {
        Self {
            num_pieces: 28,
            full_screen: false,
            image_index: None,
            screen_dims: Vec2::new(1280., 800.),
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
            params.num_pieces = usize::from_str_radix(v, 10).unwrap_or_default();
        } else if arg.starts_with("--image=") {
            let v = &arg[8..];
            params.image_index = Some(usize::from_str_radix(v, 10).unwrap_or_default());
        } else if arg.starts_with("--dims=") {
            let v = &arg[7..];
            let vec: Vec<&str> = v.split("x").collect();
            if vec.len() == 2 {
                params.screen_dims.x = vec[0].parse::<f32>().unwrap_or_default();
                params.screen_dims.y = vec[1].parse::<f32>().unwrap_or_default();
            }
        } else if arg.starts_with("--fs") {
            params.full_screen = true
        } else if arg.starts_with("--help") {
            println!(
                "these are the optional arguments:
  --pieces=n
  --dims=wxh
  --image=n
  --fs"
            );
            process::exit(if cfg!(windows) { 0x0100 } else { 0x0 });
        }
    }

    params
}
