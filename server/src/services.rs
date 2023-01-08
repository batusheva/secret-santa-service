use actix_web::{delete, get, post, put, web, Responder, HttpResponse};
use rand::Rng;
use crate::{AppState, User, Group};
use crate::models::{AdminOperationData, NewGroupData, UserData};
use super::models::{NewUserData, UpdateData};


#[get("/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder{
	
}

#[post("/users")]
async fn add_user(data: web::Data<AppState>, new_user: web::Json<NewUserData>) -> impl Responder{
    
}

#[put("/users/{id}")]
async fn update_user(data: web::Data<AppState>,path: web::Path<i32> , new_data: web::Json<UpdateData>) -> impl Responder{
   
}

#[delete("/users/{id}")]
async fn delete_user(data: web::Data<AppState>,path: web::Path<i32>) -> impl Responder{

}

#[get("/groups")]
async fn get_groups(data: web::Data<AppState>) -> impl Responder{

}

#[put("/groups/{id}/delete")]
async fn delete_group(data: web::Data<AppState>, path: web::Path<i32>, user_data: web::Json<UserData>) -> impl Responder{

}

#[post("/groups")]
async fn create_group(data: web::Data<AppState>, new_group_data: web::Json<NewGroupData>) -> impl Responder{

}

#[post("/groups/{id}/join")]
async fn join_group(data: web::Data<AppState>, path: web::Path<i32>, user_data: web::Json<UserData>) -> impl Responder{

}

#[post("/groups/{id}/leave")]
async fn leave_group(data: web::Data<AppState>, path: web::Path<i32>, user_data: web::Json<UserData>) -> impl Responder{

}

#[post("/groups/{id}/admin")]
async fn add_group_admin(data: web::Data<AppState>, path: web::Path<i32>, admin_operation_data: web::Json<AdminOperationData>) -> impl Responder{

}

#[post("/groups/{id}/unadmin")]
async fn group_unadmin(data: web::Data<AppState>, path: web::Path<i32>, admin_operation_data: web::Json<AdminOperationData>) -> impl Responder{
   
}

#[get("/groups/{id}/members")]
async fn get_group_members(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder{

}

#[get("/groups/{id}/admins")]
async fn get_group_admins(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {

}

#[post("/groups/{id}/secret-santa/start")]
async fn start_secret_santa(data: web::Data<AppState>, path: web::Path<i32>, initiator: web::Json<UserData>) -> impl Responder {
	
}

#[get("/groups/{id}/secret-santa")]
async fn get_secret_santas_list(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {

}

#[post("/groups/{id}/secret-santa")]
async fn get_secret_santa(data: web::Data<AppState>, user_data: web::Json<UserData>, path: web::Path<i32>, ) -> impl Responder {

}



pub fn users_config(cfg: &mut web::ServiceConfig){
    cfg.service(get_users)
        .service(add_user)
        .service(update_user)
        .service(delete_user)
        .service(create_group)
        .service(get_groups)
        .service(get_group_admins)
        .service(get_group_members)
        .service(group_unadmin)
        .service(add_group_admin)
        .service(join_group)
        .service(leave_group)
        .service(start_secret_santa)
        .service(get_secret_santas_list)
        .service(get_secret_santa);
}