
use std::error::Error;
// use sqlx::Connection;
use sqlx::Row;


pub struct DcUser{
    id:String,
    power:i64,
}

impl DcUser{
    pub fn DcUser(id:String)->Self{
        Self{
            id,
            power:0,
        }
    }

    pub fn moc(&self)->i64{
        return self.power;
    }
    
    pub async fn get_user_data_or_create_user(&mut self,pool:&sqlx::PgPool){

        let q = "SELECT * FROM users WHERE id = $1"; 
        let query = sqlx::query(q)
        .bind(&self.id);
    
        let row = query.fetch_optional(pool).await.unwrap();
        match row{
            Some(v)=>{
                self.power = v.get("power");
            }
            None=>{
                self.create_user(pool);
            }
        }
    }
    pub async fn create_user(&self, pool: &sqlx::PgPool){
        let query = "INSERT INTO dc_users (id, power) VALUES ($1, $2)";
        sqlx::query(query)
            .bind(&self.id)
            .bind(&self.power)
            .execute(pool)
        .await.unwrap();
    }
    pub async fn update_user(&self, pool: &sqlx::PgPool){
        // let query = "INSERT INTO dc_users (id, power) VALUES ($1, $2)";
        let query = "UPDATE dc_users SET power = $2 WHERE id = $1";
        sqlx::query(query)
            .bind(&self.id)
            .bind(&self.power)
            .execute(pool)
        .await.unwrap();
    }
}
