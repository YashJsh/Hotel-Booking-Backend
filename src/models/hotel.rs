use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

#[derive(Serialize, Deserialize)]
pub struct CreateHotel {
    pub name: String,
    pub description: String,
    pub city: String,
    pub country: String,
    pub amenities: Vec<String>, 
}


#[derive(Serialize, Deserialize)]
pub struct CreateRooms{
    pub room_number : String,
    pub room_type : String,
    pub price_per_night : BigDecimal,
    pub max_occupancy : i32
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateRoomResponse{
    pub id : String,
    pub hotel_id : Option<String>,
    pub room_number : String,
    pub room_type : String,
    pub price_per_night : BigDecimal,
    pub max_occupancy : i32
}

/*
```jsx
{
  "roomNumber": "101",
  "roomType": "Deluxe",
  "pricePerNight": 5000,
  "maxOccupancy": 2
}
```
*/