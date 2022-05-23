use uuid::Uuid;
use serde::{Serialize,Deserialize};
//use validator::Validate;



#[derive(Debug,Serialize)]
pub struct User {
    pub id :Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] //to not show the password hash
    pub password_hash : String,
    pub full_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,


}



#[derive(Debug,Serialize)]
pub struct UserLogin {
    pub id :Uuid,
    pub username: String,
    #[serde(skip_serializing)] //to not show the password hash
    pub password_hash : String,
    
 

}
//validate the input fields for new users and update users 
// looking for new ideas
    //  - strongre password
    //  - newusers

#[derive(Debug,Deserialize)]
pub struct UpdateProfile {
    pub full_name:Option<String>,
    pub bio:Option<String>,
    pub image:Option<String>,
}

#[derive(Debug,Deserialize)]
pub struct PasswordSalt {
    pub salt: String,
}