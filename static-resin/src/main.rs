mod policy;
mod filter;

fn main() {
    println!("Hello, world!");
    let g = policy::Grade::make ("malte".to_string(), 100);
}

// todo: flush out the use case (with filter objects), try to bypass it