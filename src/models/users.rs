use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignReq{
    pub username:String,
    pub password:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Users{
    pub id:Option<u32>,
    pub pid:Option<u32>,
    pub types:Option<u32>,
    pub invites:Option<String>,
    pub username:Option<String>,
    pub salt:Option<String>,
    pub password:Option<String>,
    pub go_secret:Option<String>,
    pub last_ip:Option<String>,
    pub status:Option<u8>,
    pub updated_at:Option<u32>,
    pub created_at:Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserName{
    pub id:u32,
    pub username:String,
    pub password:String,
    pub salt:String,
    pub status:u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo{
    pub id:Option<u32>,
    pub pid:Option<u32>,
    pub types:Option<u32>,
    pub invites:Option<String>,
    pub username:Option<String>,
    pub go_secret:Option<String>,
    pub last_ip:Option<String>,
    pub status:Option<u8>,
    pub updated_at:Option<u32>,
    pub created_at:Option<u32>
}