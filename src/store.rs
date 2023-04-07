use crate::schema::weights;
use crate::utils::errors::MyError;
use chrono::{NaiveDateTime, TimeZone, Utc};
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable, Clone)]
#[table_name = "weights"]
struct WeightRDB {
    weight: f32,
    timestamp: NaiveDateTime,
}

impl WeightRDB {
    fn save(self, conn: &MysqlConnection) -> Result<(), MyError> {
        diesel::insert_into(weights::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}

pub fn save(conn: &MysqlConnection, weight: f32, timestamp: i64) -> Result<(), MyError> {
    let naive = NaiveDateTime::from_timestamp_millis(timestamp * 1000).unwrap();
    println!("naive: {:?}", naive);
    WeightRDB {
        weight: weight,
        timestamp: naive,
    }
    .save(conn)?;
    Ok(())
}
