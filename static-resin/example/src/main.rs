#[macro_use]
extern crate serde_derive;

use std::fs::File;
mod grade;
use beaver::{filter, beaverio};
use beaver::policy::Policied;
use std::net;

fn main() {
    let self_ip_addr = net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1));
    let adversary_ip_addr = net::IpAddr::V4(net::Ipv4Addr::new(10, 38, 53, 87));
    let instructor_ip_addr = net::IpAddr::V4(net::Ipv4Addr::new(10, 38, 16, 198));

    let gp_malte = grade::GradePolicy { 
        student_id: "malte".to_string(),
        instructor_id: "livia".to_string(),
        student_ip: None,
        instructor_ip: Some(instructor_ip_addr.clone().to_string()), 
    };
    let gp_kinan = grade::GradePolicy { 
        student_id: "kinan".to_string(),
        instructor_id: "livia".to_string(),
        student_ip: None, 
        instructor_ip: Some(instructor_ip_addr.clone().to_string()), 
    };
    let gp_sreshtaa = grade::GradePolicy { 
        student_id: "sreshtaa".to_string(),
        instructor_id: "livia".to_string(),
        student_ip: None, 
        instructor_ip: Some(instructor_ip_addr.clone().to_string()), 
    };

    // make a protected grade object— see policy.rs for the impl of Policy on the grade
    let malte_grade = grade::PoliciedGrade::make_decomposed_unpolicied("malte".to_string(), 85, Box::new(gp_malte)); 
    let kinan_grade = grade::PoliciedGrade::make_decomposed_unpolicied("kinan".to_string(), 87, Box::new(gp_kinan));
    let mut sreshtaa_grade = grade::PoliciedGrade::make_decomposed_unpolicied("sreshtaa".to_string(), 82, Box::new(gp_sreshtaa));
    
    /***********************
        TEST EXPORT CHECK
    ************************/

    let f_malte = File::create("malte").expect("Unable to create file");
    let ctxt_malte = filter::FileContext {
        file_name: "malte".to_owned(), 
        path: "src/".to_owned(),
    };

    let mut bw_malte = beaverio::BeaverBufWriter::safe_create(f_malte, filter::Context::File(ctxt_malte));

    let mut malte_student_id = Box::new(malte_grade.student_id());
    let kinan_student_id = Box::new(kinan_grade.student_id());

    match bw_malte.safe_write_json(&Box::new(malte_grade.clone())) {
        Ok(s) => { println!("Wrote Malte's grade successfully with size: {:?}", s); },
        Err(e) => { println!("Uh oh {:?}", e); }
    } 
    match bw_malte.safe_write_json(&kinan_student_id) {
        Ok(_) => { println!("Uh oh! Security breach!"); },
        Err(e) => { println!("Successfully errored writing Kinan's grade: {:?}", e); }
    } 
    
    /*********************
        MERGE POLICIES
    **********************/

    (*malte_student_id).push_policy_str(&kinan_student_id);
    match bw_malte.safe_write_json(&malte_student_id) {
        Ok(_) => { println!("Uh oh! Security breach!"); },
        Err(e) => { println!("Successfully errored writing Malte's + Kinan's grade: {:?}", e); }
    } 

    let f_livia = File::create("livia").expect("Unable to create file");
    let ctxt_livia = filter::FileContext {
        file_name: "livia".to_owned(), 
        path: "src/".to_owned(),
    };

    let mut bw_livia = beaverio::BeaverBufWriter::safe_create(f_livia, filter::Context::File(ctxt_livia));
    match bw_livia.safe_write_json(&malte_student_id) {
        Ok(s) => { println!("Wrote Malte + Kinan's grade successfully with size: {:?}", s); },
        Err(e) => { println!("Uh oh {:?}", e); }
    } 

    /*********************
        REMOVE POLICIES
    **********************/
    let sreshtaa_student_id = Box::new(sreshtaa_grade.student_id());

    match bw_malte.safe_write_json(&sreshtaa_student_id) {
        Ok(s) => { println!("Uh oh! {:?}", s); },
        Err(e) => { println!("Successfully prevented from writing Sreshtaa's ID to Malte's file: {:?}", e); }
    }

    sreshtaa_grade.remove_policy();
    let new_student_id = Box::new(sreshtaa_grade.student_id());

    match bw_malte.safe_write_json(&new_student_id) {
        Ok(s) => { println!("Able to write Sreshtaa's data to Malte's file with size: {:?}", s); },
        Err(e) => { println!("Uh oh! {:?}", e); }
    }
    drop(bw_malte);

    /*************************
        DESERIALIZING DATA
    **************************/    
    let f_malte =  File::open("malte").unwrap();

    // Deserialize grade from Malte's file
    let mut br_deserialize = beaverio::BeaverBufReader::safe_create(f_malte);
    let malte_grade_ds: grade::PoliciedGrade = br_deserialize.safe_deserialize_line();

    // Try and write malte's grade to a new file, where it will hopefully fail the export_check
    let f_deserialize = File::create("deserialize").expect("Unable to create file");
    let ctxt_ds = filter::FileContext {
        file_name: "not_malte".to_owned(), 
        path: "src/".to_owned(),
    };

    let mut bw_not_malte = beaverio::BeaverBufWriter::safe_create(f_deserialize, filter::Context::File(ctxt_ds));

    match bw_not_malte.safe_write_json(&Box::new(malte_grade_ds)) {
        Ok(s) => { println!("Uh oh! {:?}", s); },
        Err(e) => { println!("Successfully deserialized, checked policy on Malte's data and prevented write: {:?}", e); }
    }

    /*************************
        NETWORK CONNECTIONS
    **************************/

    // Note: On your local computer, run the following command: nc -l 5000 to open a listening socket
    // Currently, if any of the sockets are not listening, the thread panics since the TCP connection failed
    // TODO: change code so that it doesn't panic
    
    let net_ctxt_sreshtaa = filter::RemoteConnectContext {
        remote_ip_address: self_ip_addr.clone(), 
        port: 5000, 
    };

    let net_ctxt_adversary = filter::RemoteConnectContext {
        remote_ip_address: adversary_ip_addr.clone(), 
        port: 5000,
    };  

    let net_ctxt_instructor = filter::RemoteConnectContext {
        remote_ip_address: instructor_ip_addr.clone(), 
        port: 5000,
    };  

    // Self Ip Address
    let sreshtaa_stream = net::TcpStream::connect(((&net_ctxt_sreshtaa).remote_ip_address, (&net_ctxt_sreshtaa).port)).unwrap();
    let mut bw_tcp_sreshtaa = beaverio::BeaverBufWriter::safe_create(sreshtaa_stream, filter::Context::ClientNetwork(net_ctxt_sreshtaa));

    match bw_tcp_sreshtaa.safe_write_json(&sreshtaa_student_id) {
        Ok(_) => { println!("Sent Sreshtaa's grade to Ip Address: {:?}", &self_ip_addr); },
        Err(e) => { println!("Uh oh! Could not send Sreshtaa's grade over the network: {:?}", e); }
    }

    // Random Ip Address
    let adv_stream = net::TcpStream::connect(((&net_ctxt_adversary).remote_ip_address, (&net_ctxt_adversary).port)).unwrap();
    let mut bw_tcp_adv = beaverio::BeaverBufWriter::safe_create(adv_stream, filter::Context::ClientNetwork(net_ctxt_adversary));

    match bw_tcp_adv.safe_write_json(&sreshtaa_student_id) {
        Ok(_) => { println!("Oh no! Incorrectly sent Sreshtaa's grade to adversary's Ip Address: {:?}", &adversary_ip_addr); },
        Err(e) => { println!("Successfully prevented sending Sreshtaa's grade to Ip Address {:?}: {:?}", &adversary_ip_addr, e); }
    }

    // Instructor's Ip Address
    let instructor_stream = net::TcpStream::connect(((&net_ctxt_instructor).remote_ip_address, (&net_ctxt_instructor).port)).unwrap();
    let mut bw_tcp_instructor = beaverio::BeaverBufWriter::safe_create(instructor_stream, filter::Context::ClientNetwork(net_ctxt_instructor));

    match bw_tcp_instructor.safe_write_json(&sreshtaa_student_id) {
        Ok(_) => { println!("Sent Sreshtaa's grades to instructor's Ip Address: {:?}", &instructor_ip_addr); },
        Err(e) => { println!("Uh oh! Could not send Sreshtaa's grade over the network: {:?}", e); }
    }
}