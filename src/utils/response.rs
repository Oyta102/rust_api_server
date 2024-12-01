use chrono::Utc;
use serde::Serialize;

#[derive(Serialize,Debug)]
pub struct ApiResp<T>{
    pub code:u32,
    pub message:Option<String>,
    pub data:T,
    pub time:u32
}

#[derive(Serialize,Debug)]
pub struct ApiNotF{
    pub code:u32,
    pub message:Option<String>,
    pub time:u32
}

impl<T> ApiResp<T>{
    pub fn success(msg: &str, data: T)->ApiResp<T>{
        ApiResp{
            message: Option::from(msg.to_string()),
            data,
            code: 200,
            time: Utc::now().timestamp() as u32
        }
    }

    pub fn error(msg: &str, data: T)->ApiResp<T>{
        ApiResp{
            message: Option::from(msg.to_string()),
            data,
            code: 100,
            time: Utc::now().timestamp() as u32
        }
    }

    pub fn auths(msg: &str,_data:T)->ApiNotF {
        ApiNotF{
            message: Option::from(msg.to_string()),
            code: 401,
            time: Utc::now().timestamp() as u32
        }
    }

    pub fn not_full(msg: &str,_data:T)->ApiNotF{
        ApiNotF{
            message: Option::from(msg.to_string()),
            code: 404,
            time: Utc::now().timestamp() as u32
        }
    }
}