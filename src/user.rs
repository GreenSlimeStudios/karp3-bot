// use std::error::Error;
// use sqlx::Connection;
use sqlx::Row;

pub struct DcUser {
    pub id: String,
    pub power: i64,
    pub last_msg: chrono::DateTime<chrono::Utc>,
    // pub last_msg: chrono::DateTime<chrono::Utc>,
}

impl DcUser {
    pub fn new(id: String) -> Self {
        Self { id, power: 0,
            last_msg: chrono::Utc::now(),
        }
    }

    pub fn moc(&self) -> i64 {
        return self.power;
    }
    pub async fn handle_passive_income(&mut self,pool: &sqlx::PgPool){
        let now = chrono::Utc::now();
        if (now.time() - self.last_msg.time()).num_seconds() > 10{
            self.last_msg = now;
            self.power+=1;
        }

        self.update_user(pool).await;
    }

    pub async fn get_user_data_or_create_user(&mut self, pool: &sqlx::PgPool) {
        let q = "SELECT * FROM db_users WHERE id = $1";
        let query = sqlx::query(q).bind(&self.id);

        let row = query.fetch_optional(pool).await.unwrap();
        match row {
            Some(v) => {
                self.power = v.get("power");
                self.last_msg = v.get("last_msg");
                println!("========={}=======",self.last_msg);
            }
            None => {
                self.create_user(pool).await;
            }
        }
    }
    pub async fn create_user(&self, pool: &sqlx::PgPool) {
        let query = "INSERT INTO db_users (id, power,last_msg) VALUES ($1, $2, $3)";
        sqlx::query(query)
            .bind(&self.id)
            .bind(&self.power)
            .bind(&self.last_msg)
            .execute(pool)
            .await
            .unwrap();
    }
    pub async fn update_user(&self, pool: &sqlx::PgPool) {
        // let query = "INSERT INTO dc_users (id, power) VALUES ($1, $2)";
        let query = "UPDATE db_users SET power = $2, last_msg = $3 WHERE id = $1";
        sqlx::query(query)
            .bind(&self.id)
            .bind(&self.power)
            .bind(&self.last_msg)
            .execute(pool)
            .await
            .unwrap();
    }
}
