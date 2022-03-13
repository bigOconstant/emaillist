use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{self, NewUser};

type DbError = Box<dyn std::error::Error + Send + Sync>;



pub fn find_user_by_id(
    uid: String,
    conn: &SqliteConnection,
) -> Result<Option<models::User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_user_by_email(
    email_in: &String,
    conn: &SqliteConnection,
) -> Result<Option<models::User>, DbError> {
use crate::schema::users::dsl::*;

println!("Finding user with email:{}",email_in);

let user = users
    .filter(email.eq(email_in.clone()))
    .first::<models::User>(conn)
    .optional()?;
Ok(user)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    nu: &NewUser,
    conn: &SqliteConnection,
) -> Result<Option<models::User>, DbError> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::users::dsl::*;

    let user_exit_check = find_user_by_email(&nu.email.clone(),conn)?;
    
    // If inserting new user and it's unsubscribed just resubscribe
    if let Some(user_exit_check) = user_exit_check {
        println!("found user! UUID{}",user_exit_check.id);
        return update_user_subscription_status(true,conn,&user_exit_check.id);
    } else{

    let new_user = models::User {
        id: Uuid::new_v4().to_string(),
        last_name:nu.lastname.clone(),
        first_name:nu.firstname.clone(),
        email:nu.email.clone(),
        subscribed:true,
    };

    println!("inserting:{}",new_user.email.clone());
    diesel::insert_into(users).values(&new_user).execute(conn)?;
    println!("done inserting now returning!");
    return find_user_by_id(new_user.id.clone(),conn);
}
}

pub fn update_user_subscription_status(
 status: bool,
 conn: &SqliteConnection,
  id_in:&String,
)-> Result<Option<models::User>, DbError>{
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(id_in))).set(subscribed.eq(status)).execute(conn)?;
    return find_user_by_id(id_in.clone(),conn);
}
