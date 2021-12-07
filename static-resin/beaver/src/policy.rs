use std::fmt;
use crate::filter;
use std::error;
use dyn_clone::DynClone;
use std::borrow::ToOwned;

extern crate beaver_derive;
use beaver_derive::Policied;

extern crate serde;
extern crate erased_serde;

// ------------------- MAIN POLICY TRAITS/STRUCTS ----------------------------------
pub trait Policy : DynClone + erased_serde::Serialize {
    fn check(&self, ctxt: &filter::Context) -> Result<(), PolicyError>; 
    fn merge(&self, _other: &Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError>;
}

dyn_clone::clone_trait_object!(Policy);
erased_serde::serialize_trait_object!(Policy);

pub trait Policied<T> : erased_serde::Serialize { // why erased serde here? 
    fn get_policy(&self) -> &Box<dyn Policy>;
    fn remove_policy(&mut self) -> ();
    fn export(self) -> T; 
    fn export_check(&self, ctxt: &filter::Context) -> Result<T, PolicyError>;
}

//erased_serde::serialize_trait_object!(Policied);

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

#[derive(Clone, Serialize)]
pub struct NonePolicy; // should NonePolicy be pub? (should people be allowed to set Policies to NonePolicy)

impl Policy for NonePolicy {
    fn check(&self, _ctxt: &filter::Context) -> Result<(), PolicyError> {
        Ok(())
    }

    fn merge(&self, other: &Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError> {
        Ok(other.clone())
    }
}

// could store a vector of policies
#[derive(Clone, Serialize)]
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
    fn check(&self, ctxt: &filter::Context) -> Result<(), PolicyError> {
        match self.policy1.check(ctxt) {
            Ok(_) => {
                match (*self.policy2).check(ctxt) {
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

#[derive(Serialize)]
//#[derive(Policied, Serialize)]
pub struct PoliciedString {
    pub(crate) inner: String, 
    policy: Box<dyn Policy>,
}

impl Policied<String> for PoliciedString {
    fn get_policy(&self) -> &Box<dyn Policy> {
        &self.policy
    }
    fn remove_policy(&mut self) -> () { self.policy = Box::new(NonePolicy); }
    fn export_check(&self, ctxt: &filter::Context) -> Result<String, PolicyError> {
        match self.get_policy().check(&ctxt) {
            Ok(_) => {
                Ok(self.inner.clone())
            }, 
            Err(pe) => { Err(pe) }
        }
    }
    fn export(self) -> String {
        self.inner
    }
}

impl PoliciedString {
    pub fn make(inner: String, policy: Box<dyn Policy>) -> PoliciedString {
        PoliciedString {
            inner, policy
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.inner.push_str(string)
    }

    pub fn push_policy_str(&mut self, policy_string: &PoliciedString) 
    -> Result<(), PolicyError> {
        match (*(self.policy)).merge(&(policy_string.policy)) {
            Ok(p) => {
                self.inner.push_str(&policy_string.inner);
                self.policy = p;
                return Ok(());
            },
            Err(pe) => { Err(pe) }
        }
        
    }
} 

impl ToOwned for PoliciedString {
    type Owned = PoliciedString;
    fn to_owned(&self) -> PoliciedString {
        PoliciedString {
            inner: self.inner.to_owned(),
            policy: self.policy.clone(),
        }
    }
}

//#[derive(Policied, Serialize)]
#[derive(Serialize)]
pub struct PoliciedNumber {
    pub(crate) inner: i64,  
    policy: Box<dyn Policy>,
}

impl Policied<i64> for PoliciedNumber {
    fn get_policy(&self) -> &Box<dyn Policy> {
        &self.policy
    }
    fn remove_policy(&mut self) -> () { self.policy = Box::new(NonePolicy); }
    fn export_check(&self, ctxt: &filter::Context) -> Result<i64, PolicyError> {
        match self.get_policy().check(&ctxt) {
            Ok(_) => {
                Ok(self.inner.clone())
            }, 
            Err(pe) => { Err(pe) }
        }
    }
    fn export(self) -> i64 {
        self.inner
    }
}

impl PoliciedNumber {
    pub fn make(inner: i64, policy: Box<dyn Policy>) -> PoliciedNumber {
        PoliciedNumber {
            inner, policy
        }
    }
} 

// Limitation: Cannot have policied containers