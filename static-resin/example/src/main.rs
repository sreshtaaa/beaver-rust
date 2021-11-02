use std::fs::File;
use std::io::{Write, Error};
mod grade;
use beaver::{policy, filter, beaverio};

fn main() {
    println!("Hello, world!");
    let gp_malte = grade::GradePolicy { student_id: "malte".to_string() };
    let gp_kinan = grade::GradePolicy { student_id: "kinan".to_string() };

    // make a protected grade object— see policy.rs for the impl of Policy on the grade
    let malte_grade = grade::Grade::make("malte".to_string(), 85, gp_malte); 
    let kinan_grade = grade::Grade::make("kinan".to_string(), 87, gp_kinan);

    // try and write to a file
    let mut f = File::create("malte").expect("Unable to create file");
    let ctxt = filter::FileContext {
        file_name: "malte".to_owned(), 
        path: "src/".to_owned(),
    };

    let mut bw = beaverio::ResinBufWriter::safe_create(f, filter::Context::File(ctxt));
    
    match bw.safe_write(&malte_grade.get_student_id()) {
        Ok(s) => { println!("Wrote Malte's grade successfully with size: {:?}", s); },
        Err(e) => { println!("Uh oh {:?}", e); }
    } 
    match bw.safe_write(&kinan_grade.get_student_id()) {
        Ok(_) => { println!("Uh oh! Security breach"); },
        Err(e) => { println!("Successfully errored writing Kinan's grade: {:?}", e); }
    } 
}

// TODO: flush out the use case (with filter objects), try to bypass it