use std::fmt;
use crate::filter;

pub trait Policied<P : Policy> {
    fn get_policy(&self) -> Box<P>;
}

pub trait Policy {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError>; 
    fn merge(&self, _other: Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError>;
}

#[derive(Debug, Clone)]
pub struct PolicyError { pub message: String }

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

pub struct PoliciedString<P : Policy> {
    string: String, 
    policy: P,
}

impl<P : Policy> PoliciedString<P> {
    pub fn make(string: String, policy: P) -> PoliciedString<P> {
        PoliciedString {
            string, policy
        }
    }
} 

impl<P : Policy + Clone> Policied<P> for PoliciedString<P> {
    fn get_policy(&self) -> Box<P> { Box::new(self.policy.clone()) }
}

impl<P : Policy> PoliciedString<P> {
    pub fn to_string(&self) -> String {
        return format!("{}", self.string);
    } // TODO: think about how to get data out such that only filter object can do so
                                // one thought: force it to call export_check
}

pub struct PoliciedNumber<P : Policy> {
    number: i64,  
    policy: P,
}

impl<P : Policy> PoliciedNumber<P> {
    pub fn make(number: i64, policy: P) -> PoliciedNumber<P> {
        PoliciedNumber {
            number, policy
        }
    }
} 

impl<P : Policy + Clone> Policied<P> for PoliciedNumber<P> {
    fn get_policy(&self) -> Box<P> { Box::new(self.policy.clone()) }
}