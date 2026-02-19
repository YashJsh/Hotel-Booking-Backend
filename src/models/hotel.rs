use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateHotel{
    pub name : String,
    pub description : String,
    pub city : String,
    pub country : String,
    pub ammenities : Vec<String>
}