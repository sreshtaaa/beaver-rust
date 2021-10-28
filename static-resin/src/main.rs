mod policyv2;
mod filter;

fn main() {
    println!("Hello, world!");
    // let g = policy::Grade::make ("malte".to_string(), 100);
    let gp = policyv2::GradePolicy { studentId: "malte".to_string() };
    let g = policyv2::Grade::make("malte".to_string(), 100, gp);
}

// todo: flush out the use case (with filter objects), try to bypass it