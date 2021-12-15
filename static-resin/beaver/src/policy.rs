use std::fmt;
use crate::filter;
use std::error;
use dyn_clone::DynClone;

extern crate beaver_derive;
use crate::derive_policied;
use crate::derive_policied_vec;
use crate::derive_policied_option;


extern crate serde;
extern crate erased_serde;
extern crate typetag;

// ------------------- MAIN POLICY TRAITS/STRUCTS ----------------------------------
#[typetag::serde(tag = "type")]
pub trait Policy : DynClone + erased_serde::Serialize {
    fn check(&self, ctxt: &filter::Context) -> Result<(), PolicyError>; 
    fn merge(&self, _other: &Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError>;
}

dyn_clone::clone_trait_object!(Policy);
// erased_serde::serialize_trait_object!(Policy);
// #[typetag::serde(tag = "type")]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct NonePolicy; // should NonePolicy be pub? (should people be allowed to set Policies to NonePolicy)

#[typetag::serde]
impl Policy for NonePolicy {
    fn check(&self, _ctxt: &filter::Context) -> Result<(), PolicyError> {
        Ok(())
    }

    fn merge(&self, other: &Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError> {
        Ok(other.clone())
    }
}

// could store a vector of policies
#[derive(Clone, Serialize, Deserialize)]
pub struct MergePolicy {
    policies: Vec<Box<dyn Policy>>
}

impl MergePolicy {
    pub fn make(policies: Vec<Box<dyn Policy>>) -> MergePolicy {
        MergePolicy { policies }
    }
}

#[typetag::serde]
impl Policy for MergePolicy {
    fn check(&self, ctxt: &filter::Context) -> Result<(), PolicyError> {
        let results: Result<Vec<_>, PolicyError> = self.policies.clone()
                                                    .into_iter()
                                                    .map(|p| p.check(ctxt))
                                                    .collect();
        match results {
            Ok(_) => Ok(()),
            Err(pe) => Err(pe)
        }
    }

    fn merge(&self, other: &Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError> {
        let mut new_policies = self.policies.clone();
        new_policies.push(other.clone());
        Ok(Box::new(MergePolicy { 
            policies: new_policies
        }))
    }
}

// ------------------- LIBRARY POLICIED STRUCTS --------------------------------------

derive_policied!(String, PoliciedString);
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

derive_policied!(i64, Policiedi64);

derive_policied_vec!(PoliciedStringVec, String, PoliciedString);

derive_policied_option!(PoliciedStringOption, String, PoliciedString);
