#[macro_use]
extern crate serde_derive;

use std::fs::File;
mod grade;
use beaver::{filter, beaverio};
use beaver::policy::Policied;

use std::net;

fn main() {
    let gp_malte = grade::GradePolicy { 
        student_id: "malte".to_string(),
        instructor_id: "livia".to_string(),
    };
    let gp_kinan = grade::GradePolicy { 
        student_id: "kinan".to_string(),
        instructor_id: "livia".to_string(),
    };
    let gp_sreshtaa = grade::GradePolicy { 
        student_id: "sreshtaa".to_string(),
        instructor_id: "livia".to_string(),
    };

    // make a protected grade object— see policy.rs for the impl of Policy on the grade
    let malte_grade = grade::Grade::make("malte".to_string(), 85, Box::new(gp_malte)); 
    let kinan_grade = grade::Grade::make("kinan".to_string(), 87, Box::new(gp_kinan));
    let mut sreshtaa_grade = grade::Grade::make("sreshtaa".to_string(), 82, Box::new(gp_sreshtaa));
    
    /***********************
        TEST EXPORT CHECK
    ************************/

    let f_malte = File::create("malte").expect("Unable to create file");
    let ctxt_malte = filter::FileContext {
        file_name: "malte".to_owned(), 
        path: "src/".to_owned(),
    };

    let mut bw_malte = beaverio::BeaverBufWriter::safe_create(f_malte, filter::Context::File(ctxt_malte));

    let mut malte_student_id = Box::new(malte_grade.get_student_id());
    let mut kinan_student_id = Box::new(kinan_grade.get_student_id());

    match bw_malte.safe_write(&malte_student_id) {
        Ok(s) => { println!("Wrote Malte's grade successfully with size: {:?}", s); },
        Err(e) => { println!("Uh oh {:?}", e); }
    } 
    match bw_malte.safe_write(&kinan_student_id) {
        Ok(_) => { println!("Uh oh! Security breach!"); },
        Err(e) => { println!("Successfully errored writing Kinan's grade: {:?}", e); }
    } 
    
    /*********************
        MERGE POLICIES
    **********************/

    (*malte_student_id).push_policy_str(&kinan_student_id);
    match bw_malte.safe_write(&malte_student_id) {
        Ok(_) => { println!("Uh oh! Security breach!"); },
        Err(e) => { println!("Successfully errored writing Malte's + Kinan's grade: {:?}", e); }
    } 

    let f_livia = File::create("livia").expect("Unable to create file");
    let ctxt_livia = filter::FileContext {
        file_name: "livia".to_owned(), 
        path: "src/".to_owned(),
    };

    let mut bw_livia = beaverio::BeaverBufWriter::safe_create(f_livia, filter::Context::File(ctxt_livia));
    match bw_livia.safe_write(&malte_student_id) {
        Ok(s) => { println!("Wrote Malte + Kinan's grade successfully with size: {:?}", s); },
        Err(e) => { println!("Uh oh {:?}", e); }
    } 

    /*********************
        REMOVE POLICIES
    **********************/
    let sreshtaa_student_id = Box::new(sreshtaa_grade.get_student_id());

    match bw_malte.safe_write(&sreshtaa_student_id) {
        Ok(s) => { println!("Uh oh! {:?}", s); },
        Err(e) => { println!("Successfully prevented from writing Sreshtaa's ID to Malte's file: {:?}", e); }
    }

    sreshtaa_grade.remove_policy();
    let new_student_id = Box::new(sreshtaa_grade.get_student_id());

    match bw_malte.safe_write(&new_student_id) {
        Ok(s) => { println!("Able to write Sreshtaa's data to Malte's file with size: {:?}", s); },
        Err(e) => { println!("Uh oh! {:?}", e); }
    }

    // dev mistake: try to get student_id field out without policy

    // malicious dev: try to change policy 
    // pub struct EmptyPolicy

    /*************************
        NETWORK CONNECTIONS
    **************************/
    let ip_addr = net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1));
    let client_ctxt_sreshtaa = filter::RemoteConnectContext {
        remote_ip_address: ip_addr.clone(), 
        port: 29290,
    };

    println!("Ip Address: {}, Port: {}", (&client_ctxt_sreshtaa).remote_ip_address, (&client_ctxt_sreshtaa).port);

    let mut stream = net::TcpStream::connect(((&client_ctxt_sreshtaa).remote_ip_address, (&client_ctxt_sreshtaa).port)).unwrap();
    let mut bw_tcp_sreshtaa = beaverio::BeaverBufWriter::safe_create(stream, filter::Context::ClientNetwork(client_ctxt_sreshtaa));

    match bw_tcp_sreshtaa.safe_write(&sreshtaa_student_id) {
        Ok(ip) => { println!("Sent Sreshtaa's grade to Ip Address: {:?}", ip); },
        Err(e) => { println!("Uh oh! Could not send Sreshtaa's grade over the network: {:?}", e); }
    }
}

// TODO: flush out the use case (with filter objects), try to bypass it