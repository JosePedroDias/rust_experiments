// https://doc.rust-lang.org/rust-by-example/trait/iter.html
/* 
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}


fn main() {
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }
}
 */


// EX 2 /////////////////

/*
struct LinearF32 {
    v: f32,
    duration: f32,
    t: f32,
    delta_t: f32,
    delta_v: f32
}

impl LinearF32 {
    fn new(start_v:f32, end_v:f32, duration:f32) -> LinearF32 {
        let fps = 60f32;
        let delta_t = 1f32 / fps;
        LinearF32 {
            v: start_v,
            duration,
            t: 0.,
            delta_t,
            delta_v: (end_v - start_v) / (fps * duration)
        }
    }
}

impl Iterator for LinearF32 {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.t += self.delta_t;
        if self.t > self.duration {
            return None
        }
        self.v += self.delta_v;

        Some(self.v)
    }
}

fn main() {
    let iter = LinearF32::new(3.0, 1.0, 0.25);
    for v in iter {
        println!("> {}", v);
    }
}
*/

use genawaiter::sync::{Gen, GenBoxed};

fn countdown(start: i32) -> GenBoxed<i32> {
    Gen::new_boxed(|co| {
        let mut i: i32;
        i = start;
        async move {
            loop {
                if i == 0 {
                    return;
                }

                i -= 1;
    
                co.yield_(i).await;
            }
        }
    })
}

fn main() {
    for n in countdown(10) {
        println!("{}", n);
    }    
}
