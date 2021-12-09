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
    fn make(inner: T, policy: Box<dyn Policy>) -> Self;
    fn get_policy(&self) -> &Box<dyn Policy>;
    fn remove_policy(&mut self) -> ();
    fn export(&self) -> T; 
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

// ------------------- LIBRARY POLICIED STRUCTS --------------------------------------

//derive_policied!(String, PoliciedString);

#[derive(Serialize)]
//#[derive(Policied, Serialize)]
pub struct PoliciedString {
    pub(crate) inner: String, 
    policy: Box<dyn Policy>,
}

impl Policied<String> for PoliciedString {
    fn make(inner: String, policy: Box<dyn Policy>) -> PoliciedString {
        PoliciedString {
            inner, policy
        }
    }
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
    fn export(&self) -> String {
        self.inner.clone()
    }
}

impl PoliciedString {
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
    fn make(inner: i64, policy: Box<dyn Policy>) -> PoliciedNumber {
        PoliciedNumber {
            inner, policy
        }
    }
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
    fn export(&self) -> i64 {
        self.inner.clone()
    }
}

//derive_policied_vec!(PoliciedStringVec, String, PoliciedString);
#[derive(Serialize)]
pub struct PoliciedStringVec {
    inner: Vec<String>,
    policy: Box<dyn Policy>,
}

impl Policied<Vec<String>> for PoliciedStringVec {
    fn make(inner: Vec<String>, policy: Box<dyn Policy>) -> PoliciedStringVec {
        PoliciedStringVec {
            inner, policy
        }
    }
    fn get_policy(&self) -> &Box<dyn Policy> {
        &self.policy
    }
    fn remove_policy(&mut self) -> () { self.policy = Box::new(NonePolicy); }
    fn export_check(&self, ctxt: &filter::Context) -> Result<Vec<String>, PolicyError> {
        match self.get_policy().check(&ctxt) {
            Ok(_) => {
                Ok(self.inner.clone())
            }, 
            Err(pe) => { Err(pe) }
        }
    }
    fn export(&self) -> Vec<String> {
        self.inner.clone()
    }
}

impl PoliciedStringVec {
    pub fn push(&mut self, value: String) {
        self.inner.push(value);
    }

    pub fn push_policy(&mut self, value: PoliciedString) {
        self.policy = self.policy.merge(value.get_policy()).unwrap();
        self.inner.push(value.export());
    }

    pub fn pop(&mut self) -> Option<PoliciedString> {
        match self.inner.pop() {
            Some(v) => Some(PoliciedString { inner: v, policy: self.policy.clone() }),
            None => None
        }
    }
}

//derive_policied_option!(PoliciedStringOption, String, PoliciedString);
#[derive(Serialize)]
pub struct PoliciedStringOption {
    inner: Option<String>,
    policy: Box<dyn Policy>,
}

impl Policied<Option<String>> for PoliciedStringOption {
    fn make(inner: Option<String>, policy: Box<dyn Policy>) -> PoliciedStringOption {
        PoliciedStringOption {
            inner, policy
        }
    }
    fn get_policy(&self) -> &Box<dyn Policy> {
        &self.policy
    }
    fn remove_policy(&mut self) -> () { self.policy = Box::new(NonePolicy); }
    fn export_check(&self, ctxt: &filter::Context) -> Result<Option<String>, PolicyError> {
        match self.get_policy().check(&ctxt) {
            Ok(_) => {
                Ok(self.inner.clone())
            }, 
            Err(pe) => { Err(pe) }
        }
    }
    fn export(&self) -> Option<String> {
        self.inner.clone()
    }
}

impl PoliciedStringOption {
    pub fn make_option(ops: Option<PoliciedString>) -> Self {
        match ops {
            Some(s) => PoliciedStringOption {
                inner: Some(s.export()),
                policy: s.policy.clone()
            },
            None => PoliciedStringOption {
                inner: None,
                policy: Box::new(NonePolicy)
            }
        }
    }

    pub fn get_option(self) -> Option<PoliciedString> {
        match self.inner {
            Some(s) => Some(PoliciedString::make(s, self.policy.clone())),
            None => None
        }
    }
}
