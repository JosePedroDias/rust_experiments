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

// https://crates.io/crates/genawaiter
// https://docs.rs/genawaiter/0.99.1/genawaiter/
// https://github.com/whatisaphone/genawaiter

//use genawaiter::sync::{Gen, GenBoxed};

/* fn linear_gen(start_t: f32, start_v:f32, end_v:f32, duration:f32) -> GenBoxed<f32> {
    let start_t = start_t;
    let end_t = start_v + duration;
    let mut t = start_t;
    let mut v = start_v;

    let mut generator = gen!({
        yield_!(10);
    });

    Gen::new_boxed(|co| {
        async move {
            t = co.yield_(v).await;
            loop {
                if i == 0 {
                    return;
                }

                i -= 1;
    
                co.yield_(i).await;
            }
        }
    })
} */

// https://github.com/whatisaphone/genawaiter/blob/master/tests/sync.rs

/* use genawaiter::{sync::gen, yield_, GeneratorState};

fn main() {
    let mut gen = gen!({
        let a = yield_!(10_u8);
        println!("gen got {}", a);

        let a = yield_!(20_u8);
        println!("gen got {}", a);

        let a = yield_!(30_u8);
        println!("gen got {}", a);
    });

    // 1st call

    let v = gen.resume_with("ignored");

    if let GeneratorState::Yielded(num) = v {
        println!("xxx {}", num)
    }

    match v {
        GeneratorState::Yielded(num) => println!("yyy {}", num),
        GeneratorState::Complete(()) => ()
    }

    assert_eq!(v, GeneratorState::Yielded(10_u8));

    // 2nd call

    let v = gen.resume_with("b");
    assert_eq!(v, GeneratorState::Yielded(20_u8));

    // 3rd call
    
    let v = gen.resume_with("c");
    assert_eq!(v, GeneratorState::Yielded(30_u8));

    // 4th call

    let v = gen.resume_with("d");
    assert_eq!(v, GeneratorState::Complete(()));
}
 */

use genawaiter::{sync::gen, yield_, GeneratorState};

// TODO make generator be parameterizable (wrap it inside a function)
fn main() {
    let mut gen = gen!({
        let a = yield_!(10_u8);
        println!("gen got {}", a);

        let a = yield_!(20_u8);
        println!("gen got {}", a);

        let a = yield_!(30_u8);
        println!("gen got {}", a);
    });

    // 1st call

    let v = gen.resume_with("ignored");

    if let GeneratorState::Yielded(num) = v {
        println!("xxx {}", num)
    }

    match v {
        GeneratorState::Yielded(num) => println!("yyy {}", num),
        GeneratorState::Complete(()) => ()
    }

    assert_eq!(v, GeneratorState::Yielded(10_u8));

    // 2nd call

    let v = gen.resume_with("b");
    assert_eq!(v, GeneratorState::Yielded(20_u8));

    // 3rd call
    
    let v = gen.resume_with("c");
    assert_eq!(v, GeneratorState::Yielded(30_u8));

    // 4th call

    let v = gen.resume_with("d");
    assert_eq!(v, GeneratorState::Complete(()));
}
