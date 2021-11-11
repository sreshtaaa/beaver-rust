use std::fs::File;
mod grade;
use beaver::{filter, beaverio};

fn main() {
    let gp_malte = grade::GradePolicy { 
        student_id: "malte".to_string(),
        instructor_id: "livia".to_string(),
    };
    let gp_kinan = grade::GradePolicy { 
        student_id: "kinan".to_string(),
        instructor_id: "livia".to_string(),
    };

    // make a protected grade object— see policy.rs for the impl of Policy on the grade
    let malte_grade = grade::Grade::make("malte".to_string(), 85, Box::new(gp_malte)); 
    let kinan_grade = grade::Grade::make("kinan".to_string(), 87, Box::new(gp_kinan));

    // try and write to a file
    let f_malte = File::create("malte").expect("Unable to create file");
    let ctxt_malte = filter::FileContext {
        file_name: "malte".to_owned(), 
        path: "src/".to_owned(),
    };

    let mut bw_malte = beaverio::ResinBufWriter::safe_create(f_malte, filter::Context::File(ctxt_malte));

    let malte_student_id = malte_grade.get_student_id();
    let kinan_student_id = kinan_grade.get_student_id();

    match bw_malte.safe_write(&malte_student_id) {
        Ok(s) => { println!("Wrote Malte's grade successfully with size: {:?}", s); },
        Err(e) => { println!("Uh oh {:?}", e); }
    } 
    match bw_malte.safe_write(&kinan_student_id) {
        Ok(_) => { println!("Uh oh! Security breach!"); },
        Err(e) => { println!("Successfully errored writing Kinan's grade: {:?}", e); }
    } 
    
    // try to merge policies
    malte_student_id.push_policy_str(kinan_student_id);
    match bw_malte.safe_write(&malte_student_id) {
        Ok(_) => { println!("Uh oh! Security breach!"); },
        Err(e) => { println!("Successfully errored writing Malte's + Kinan's grade: {:?}", e); }
    } 

    let f_livia = File::create("livia").expect("Unable to create file");
    let ctxt_livia = filter::FileContext {
        file_name: "livia".to_owned(), 
        path: "src/".to_owned(),
    };
    let mut bw_livia = beaverio::ResinBufWriter::safe_create(f_livia, filter::Context::File(ctxt_livia));
    match bw_livia.safe_write(&malte_student_id) {
        Ok(s) => { println!("Wrote Malte + Kinan's grade successfully with size: {:?}", s); },
        Err(e) => { println!("Uh oh {:?}", e); }
    } 

    // dev mistake: try to get student_id field out without policy

    // malicious dev: try to change policy 
    // pub struct EmptyPolicy
}

// TODO: flush out the use case (with filter objects), try to bypass it