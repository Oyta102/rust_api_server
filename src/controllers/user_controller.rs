use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use crate::utils::crypto::{generate_private_key, hash_pass, verify_pass};
use crate::models::users::{SignReq, UserInfo, UserName, Users};
use crate::models::wallet::{UsersWallet, WalletInfo};
use crate::utils::jwt::{create_jwt};
use crate::utils::rands::generate_random_string;
use crate::utils::response::ApiResp;
use actix_web::web::Data;
use sqlx::MySqlPool;
use rust_decimal::Decimal;
use chrono::Utc;

#[post("/auth/signup")]
pub async fn signup(pool:Data<MySqlPool>,req:web::Json<SignReq>,req_ip:HttpRequest)->impl Responder{
    let now = Utc::now().timestamp();
    let ip = req_ip.peer_addr().unwrap().ip();
    let salt = generate_random_string(10);
    let hash_password = hash_pass(&req.password,&salt);
    let private_key = generate_private_key();

    let exis_users = sqlx::query_as!(
        UserName,
        "SELECT id, username, password, salt, status FROM oyta_users WHERE username=?",
        req.username
    )
        .fetch_one(pool.get_ref())
        .await;

    if let Ok(_) = exis_users{
       return HttpResponse::Conflict().json(ApiResp::error("用户已存在", ()))
    }

    let users = Users{
        id: Some(0),
        pid: Some(0),
        types: Some(1),
        invites: None,
        username: Option::from(req.username.to_string()),
        salt: Option::from(salt),
        password: Option::from(hash_password),
        go_secret: None,
        last_ip: Option::from(ip.to_string()),
        status: Some(0),
        updated_at: Some(now as u32),
        created_at: Some(now as u32),
    };

    let result = sqlx::query!(
        "INSERT INTO oyta_users (pid,types,invites,username,salt,password,go_secret,last_ip,status,created_at,updated_at) VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        users.pid,
        users.types,
        users.invites,
        users.username,
        users.salt,
        users.password,
        users.go_secret,
        users.last_ip,
        users.status,
        users.created_at,
        users.updated_at
    )
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(result)=>{
            let user_id = result.last_insert_id();
            //let snum = format!("{:.4}"xx);
            let waller = UsersWallet{
                id: 0,
                uid: user_id as u32,
                coin: Decimal::from(0),
                dcoin: Decimal::from(0),
                bonus: Decimal::from(0),
                score: Some(0),
                secret_key: Option::from(private_key),
                btc_add: None,
                ltc_add: None,
                eth_add: None,
                tron_add: None,
                updated_at: Some(now as u32),
                created_at: Some(now as u32),
            };

            let wallet_result = sqlx::query!(
                "INSERT INTO oyta_users_wallet (uid,coin,dcoin,bonus,score,secret_key,btc_add,ltc_add,eth_add,tron_add,updated_at,created_at) VALUES (?,?,?,?,?,?,?,?,?,?,?,?)",
                waller.uid,
                waller.coin,
                waller.dcoin,
                waller.bonus,
                waller.score,
                waller.secret_key,
                waller.btc_add,
                waller.ltc_add,
                waller.eth_add,
                waller.tron_add,
                waller.updated_at,
                waller.created_at,
            )
                .execute(pool.get_ref())
                .await;

            match wallet_result {
                Ok(_)=>HttpResponse::Created().json(ApiResp::success("注册成功",())),
                Err(_)=>HttpResponse::InternalServerError().json(ApiResp::error("注册失败", ()))
            }
        }
        Err(_)=>HttpResponse::InternalServerError().json(ApiResp::error("注册失败", ()))
    }

}
#[post("/auth/signin")]
pub async fn signin(pool:Data<MySqlPool>,req:web::Json<SignReq>,req_ip:HttpRequest)->impl Responder{
    let now = Utc::now().timestamp();
    let ip = req_ip.peer_addr().unwrap().ip();
    let user = sqlx::query_as!(
        UserName,
        "SELECT id, username, password, salt, status FROM oyta_users WHERE username = ?"
        ,req.username
    )
        .fetch_one(pool.get_ref())
        .await;

    match user {
        Ok(user) if verify_pass(&req.password,&user.password,&user.salt)=>{
            if user.status == 1{
                return HttpResponse::Forbidden().json(ApiResp::error("用户已被禁用",()));
            }

            sqlx::query!(
                "UPDATE oyta_users SET last_ip = ?, updated_at = ? WHERE id = ?",
                ip.to_string(),now as u32,user.id
            )
                .execute(pool.get_ref())
                .await.expect("TODO: panic message");

            let token = create_jwt(&user).unwrap();
            HttpResponse::Ok().json(ApiResp::success("登录成功", serde_json::json!({ "token": token })))
        }
        Ok(_)=>HttpResponse::Unauthorized().json(ApiResp::error("密码错误",())),
        Err(_)=>HttpResponse::NotFound().json(ApiResp::error("用户不存在",()))
    }
}
#[get("/users/info")]
pub async fn info(pool: Data<MySqlPool>,req:HttpRequest)->impl Responder{
    let &users_id = req.extensions().get::<u32>().unwrap();
    let user = sqlx::query_as!(
        UserInfo,
        "SELECT id,pid,types,invites,username,go_secret,last_ip,status,updated_at,created_at FROM oyta_users WHERE id = ?",
        users_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match user {
        Ok(user)=>HttpResponse::Ok().json(ApiResp::success("获取成功", user)),
        Err(_)=>HttpResponse::Unauthorized().json(ApiResp::auths("用户不存在",()))
    }

}
#[post("/users/info/edit")]
pub async fn edit_info(_pool:Data<MySqlPool>,_req:web::Json<Users>)->impl Responder{
    HttpResponse::Ok().json(ApiResp::success("OK",()))
}

#[get("/users/info/wallet")]
pub async fn wallet(pool:Data<MySqlPool>,req:HttpRequest)->impl Responder{
    let &users_id = req.extensions().get::<u32>().unwrap();
    let wallet = sqlx::query_as!(
        WalletInfo,
        "SELECT id,uid,coin,dcoin,bonus,score,btc_add,ltc_add,eth_add,tron_add FROM oyta_users_wallet WHERE uid = ?",
        users_id
    )
        .fetch_one(pool.get_ref())
        .await;
    //from_str(&user.coin.to_string()).unwrap()
    match wallet {
        Ok(wallet)=>HttpResponse::Ok().json(ApiResp::success("获取成功",wallet)),
        Err(_)=>HttpResponse::NotFound().json(ApiResp::error("用户不存在",()))
    }
}