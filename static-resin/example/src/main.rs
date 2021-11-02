use std::fs::File;
use std::io::{Write, Error};
mod grade;
use beaver::{policy, filter, ResinBufWriter};

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

    let mut bw = ResinBufWriter::ResinBufWriter::safe_create(f, filter::Context::File(ctxt));
    
    bw.safe_write(&malte_grade.get_student_id()); // this should return Ok(usize)
    bw.safe_write(&kinan_grade.get_student_id()); // this should panic
}

// TODO: think about how to get data out such that only filter object can do so
                                   // one thought: force it to call export_check
                                   // problem: how to get information from StringablePolicy in a protected way?
// check rust access modifiers and what impact it has on the code. 

// TODO: implement this so that it returns either an error::Error or an io::Error and not panic
// TODO: flush out the use case (with filter objects), try to bypass it