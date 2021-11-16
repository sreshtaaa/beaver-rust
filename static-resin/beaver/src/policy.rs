use std::fmt;
use crate::filter;
use std::error;
use dyn_clone::DynClone;
extern crate beaver_derive;
use beaver_derive::Policied;

// ------------------- MAIN POLICY TRAITS/STRUCTS ----------------------------------
pub trait Policy : DynClone {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError>; 
    fn merge(&self, _other: &Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError>;
}

dyn_clone::clone_trait_object!(Policy);

pub trait Policied {
    fn get_policy(&self) -> &Box<dyn Policy>;
    fn remove_policy(&mut self) -> (); // this assumes that the policy is named policy... is that ok?
}

#[derive(Debug, Clone)]
pub struct PolicyError { pub message: String }

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl error::Error for PolicyError {
    fn description(&self) -> &str {
        &self.message
    }
}

// ------------------- LIBRARY POLICY STRUCTS --------------------------------------

#[derive(Clone)]
pub struct NonePolicy; // should NonePolicy be pub? (should people be allowed to set Policies to NonePolicy)

impl Policy for NonePolicy {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError> {
        Ok(())
    }

    fn merge(&self, other: &Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError> {
        Ok(other.clone())
    }
}

// could store a vector of policies
#[derive(Clone)]
pub struct MergePolicy {
    policy1: Box<dyn Policy>,
    policy2: Box<dyn Policy>,
}

impl MergePolicy {
    pub fn make(policy1: Box<dyn Policy>, policy2: Box<dyn Policy>) -> MergePolicy {
        MergePolicy { policy1, policy2 }
    }
}

impl Policy for MergePolicy {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError> {
        match self.policy1.export_check(ctxt) {
            Ok(_) => {
                match (*self.policy2).export_check(ctxt) {
                    Ok(_) => { Ok(()) },
                    Err(pe) => { Err(pe) }
                }
            },
            Err(pe) => { Err(pe) }
        }
    }

    fn merge(&self, other: &Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError> {
        Ok(Box::new(MergePolicy { 
            policy1: Box::new(self.clone()),
            policy2: other.clone(),
        }))
    }
}

#[derive(Policied)]
pub struct PoliciedString {
    pub(crate) string: String, 
    policy: Box<dyn Policy>,
}

impl PoliciedString {
    pub fn make(string: String, policy: Box<dyn Policy>) -> PoliciedString {
        PoliciedString {
            string, policy
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.string.push_str(string)
    }

    pub fn push_policy_str(&mut self, policy_string: &PoliciedString) 
    -> Result<(), PolicyError> {
        match (*(self.policy)).merge(&(policy_string.policy)) {
            Ok(p) => {
                self.string.push_str(&policy_string.string);
                self.policy = p;
                return Ok(());
            },
            Err(pe) => { Err(pe) }
        }
        
    }
} 

#[derive(Policied)]
pub struct PoliciedNumber {
    pub(crate) number: i64,  
    policy: Box<dyn Policy>,
}

impl PoliciedNumber {
    pub fn make(number: i64, policy: Box<dyn Policy>) -> PoliciedNumber {
        PoliciedNumber {
            number, policy
        }
    }
} 
