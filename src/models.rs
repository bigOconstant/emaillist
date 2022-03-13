use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub email: String,
    pub subscribed:bool,
    pub first_name:String,
    pub last_name:String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub firstname:String,
    pub lastname:String,
}


#[derive(Serialize, Deserialize)]
pub struct SubscribeForm {
    pub subscribed: Option<String>,
    pub id:String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppContext {
    pub app_name:String,
}
