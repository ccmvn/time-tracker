// Defines the authority enum and checks if a user has the required authority
// This module is used by the admin panel to check if a user has the required authority to access a page

use std::error::Error;

#[derive(PartialEq)]
enum Authority {
    Employee,
    Administrator,
}

impl Authority {
    fn from_str(s: &String) -> Option<Self> {
        match s.as_str() {
            "EMPLOYEE" => Some(Authority::Employee),
            "ADMINISTRATOR" => Some(Authority::Administrator),
            _ => None,
        }
    }
}

pub fn check_authority(user_authority: &String, required_authority: &String) -> Result<(), Box<dyn Error>> {
    let user_authority = Authority::from_str(user_authority).ok_or("Invalid user authority")?;
    let required_authority = Authority::from_str(required_authority).ok_or("Invalid required authority")?;

    if user_authority == required_authority {
        Ok(())
    } else {
        Err("User does not have the required authority".into())
    }
}
