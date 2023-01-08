use actix_web::{delete, get, post, put, web, Responder, HttpResponse};
use rand::Rng;
use crate::{AppState, User, Group};
use crate::models::{AdminOperationData, NewGroupData, UserData};
use super::models::{NewUserData, UpdateData};


#[get("/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder{
    HttpResponse::Ok().json(data.users_list.lock().unwrap().to_vec())
}
 
#[post("/users")]
async fn add_user(data: web::Data<AppState>, new_user: web::Json<NewUserData>) -> impl Responder{
    let mut users_list = data.users_list.lock().unwrap();
    let mut max_id = 0;
    for i in 0..users_list.len() {
        if users_list[i].id > max_id{
            max_id = users_list[i].id;
        }
    }
 
    users_list.push(User{
        id: max_id + 1,
        name: new_user.name.clone()
    });
 
    HttpResponse::Ok().json(users_list.to_vec())
}
 
#[put("/users/{id}")]
async fn update_user(data: web::Data<AppState>,path: web::Path<i32> , new_data: web::Json<UpdateData>) -> impl Responder{
 
    let id = path.into_inner();
    let mut users_list = data.users_list.lock().unwrap();
 
    for i in 0..users_list.len() {
        if users_list[i].id == id {
            users_list[i].name = new_data.title.clone();
            break;
        }
    }
 
    HttpResponse::Ok().json(users_list.to_vec())
}
 
#[delete("/users/{id}")]
async fn delete_user(data: web::Data<AppState>,path: web::Path<i32>) -> impl Responder{
 
    let id = path.into_inner();
    let mut users_list = data.users_list.lock().unwrap();
 
 
    for i in 0..users_list.len() {
        if users_list[i].id == id {
            users_list.remove(i);
            break;
        }
    }
 
    HttpResponse::Ok().json(users_list.to_vec())
}
 
#[get("/groups")]
async fn get_groups(data: web::Data<AppState>) -> impl Responder{
 
    HttpResponse::Ok().json(data.groups_list.lock().unwrap().to_vec())
}

#[put("/groups/{id}/delete")]
async fn delete_group(data: web::Data<AppState>, path: web::Path<i32>, user_data: web::Json<UserData>) -> impl Responder{
 
    let mut groups_list = data.groups_list.lock().unwrap();
    let group_id = path.into_inner();
 
    for i in 0..groups_list.len() {
 
        if groups_list[i].id == group_id{
 
            let mut initiator_is_admin = false;
 
            for j in 1..groups_list[i].admins_list.len() {
 
                if groups_list[i].admins_list[j] == user_data.id {
                    initiator_is_admin = true;
                    break;
                }
 
            }
 
            if initiator_is_admin {
                groups_list.remove(i)
            }
 
            break;
        }
    }
 
    HttpResponse::Ok().json(groups_list.to_vec())
}

#[post("/groups")]
async fn create_group(data: web::Data<AppState>, new_group_data: web::Json<NewGroupData>) -> impl Responder{
 
    let mut groups_list = data.groups_list.lock().unwrap();
    let mut max_id:usize = 0;
    for i in 0..groups_list.len() {
        if groups_list[i].id > max_id as i32{
            max_id = groups_list[i].id as usize;
        }
    }
 
    groups_list.push(Group{
        id: (max_id + 1) as i32,
        name: new_group_data.name.clone(),
        admins_list: vec![new_group_data.creator_id.clone()],
        members_list: vec![new_group_data.creator_id.clone()],
        secret_santa_list: vec![],
        is_open: true
    });
 
    HttpResponse::Ok().json(groups_list.to_vec())
}

#[post("/groups/{id}/join")]
async fn join_group(data: web::Data<AppState>, path: web::Path<i32>, user_data: web::Json<UserData>) -> impl Responder{
 
    let mut groups_list = data.groups_list.lock().unwrap();
    let group_id = path.into_inner();
 
    for i in 0..groups_list.len() {
 
        if groups_list[i].id == group_id{
 
            if !groups_list[i].is_open {break;}
 
            let mut user_not_in_group = true;
 
            for j in 0..groups_list[i].members_list.len() {
 
                if groups_list[i].members_list[j] == user_data.id {
                    user_not_in_group = false;
                    break;
                }
 
            }
 
            if user_not_in_group {
                groups_list[i].members_list.push(user_data.id);
            }
 
            break;
        }
 
    }
 
    HttpResponse::Ok().json(groups_list.to_vec())
}

#[post("/groups/{id}/leave")]
async fn leave_group(data: web::Data<AppState>, path: web::Path<i32>, user_data: web::Json<UserData>) -> impl Responder{
 
    let mut groups_list = data.groups_list.lock().unwrap();
    let group_id = path.into_inner();
 
    for i in 0..groups_list.len() {
 
        if groups_list[i].id == group_id{
 
            if !groups_list[i].is_open {break;}
            if groups_list[i].admins_list.len() == 1 && groups_list[i].admins_list[i] == user_data.id {break; }
 
            for j in 1..groups_list[i].admins_list.len() {
 
                if groups_list[i].admins_list[j] == user_data.id {
                    groups_list[i].admins_list.remove(j);
                }
 
            }
 
            for j in 0..groups_list[i].members_list.len() {
 
                if groups_list[i].members_list[j] == user_data.id {
                    groups_list[i].members_list.remove(j);
                }
 
            }
 
            break;
 
        }
    }
 
    HttpResponse::Ok().json(groups_list.to_vec())
}

#[post("/groups/{id}/admin")]
async fn add_group_admin(data: web::Data<AppState>, path: web::Path<i32>, admin_operation_data: web::Json<AdminOperationData>) -> impl Responder{
 
    let mut groups_list = data.groups_list.lock().unwrap();
    let group_id = path.into_inner();
 
    for i in 0..groups_list.len() {
 
        if groups_list[i].id == group_id{
 
            let mut initiator_is_admin = false;
            let mut candidate_in_group = false;
 
            for j in 1..groups_list[i].admins_list.len() {
 
                if groups_list[i].admins_list[j] == admin_operation_data.initiator_id {
                    initiator_is_admin = true;
                    break;
                }
 
            }
 
            for j in 0..groups_list[i].members_list.len() {
 
                if groups_list[i].members_list[j] == admin_operation_data.candidate_id {
                    candidate_in_group = true;
                    break;
                }
 
            }
 
            if initiator_is_admin && candidate_in_group {
                groups_list[i].admins_list.push(admin_operation_data.candidate_id);
            }
 
            break;
        }
    }
 
    HttpResponse::Ok().json(groups_list[group_id as usize].admins_list.to_vec())
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
