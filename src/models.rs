use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::users;
use super::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)] 
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String,
    pub fio: String,
    pub unik: String,
    pub grade: String,
    pub birthdate: String,
}

#[derive(Serialize, Deserialize, Insertable)] 
#[table_name = "users"]
pub struct UserData {
    pub login: String,
    pub password: String,
    pub fio: String,
    pub unik: String,
    pub grade: String,
    pub birthdate: String,
}

#[derive(Serialize, Deserialize)] 
pub struct UserLoginData {
    pub login: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub login: String,
    pub password: String,
}

impl User {

    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(new_user: NewUser, conn: &PgConnection) -> bool {

        let user:UserData = UserData{ login: new_user.login, password: new_user.password, fio:String::from(""), grade:String::from(""), unik:String::from(""), birthdate:String::from("")};
        return diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_login_password(user: UserLoginData, conn: &PgConnection) -> User {
        all_users
            .filter(users::login.eq(user.login))
            .filter(users::password.eq(user.password))
            .first(conn)
            .expect("error!")
    }

    pub fn update_user(user: UserData, conn: &PgConnection) -> bool  {
        let users_to_update = all_users
            .filter(users::login.eq(user.login))
            .filter(users::password.eq(user.password));

        return diesel::update(users_to_update)
            .set((users::fio.eq(user.fio), users::unik.eq(user.unik), users::grade.eq(user.grade), users::birthdate.eq(user.birthdate)))
            .execute(conn)
            .is_ok()
    }
}