use sqlx::{Pool, Sqlite};
use rocket::request::{FromRequest, Outcome, Request};

// #[derive(Debug)]
// pub struct DBPool{
//     pool: Pool<Sqlite>
// }

// impl DBPool{
//     pub fn new(pl: Pool<Sqlite>)->Self{
//         DBPool{
//             pool: pl
//         }
//     }
// }

// #[derive(Debug)]
// pub struct DBError{
//     msg: String
// }

// #[rocket::async_trait]
// impl <'r> FromRequest<'r> for DBPool {
//     type Error = DBError;

//     async fn from_request(req: &'r Request<'_>)-> Outcome<Self, Self::Error>{
        
//     }
// }
