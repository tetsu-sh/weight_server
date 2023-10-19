use crate::schema::weights::{self, timestamp};
use crate::utils::errors::MyError;
use crate::FetchWeightResponse;
use crate::Weight;
use chrono::{DateTime as CDateTime, Local, TimeZone};
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, sql_query};

#[derive(Debug, Queryable, Insertable, Clone)]
#[table_name = "weights"]
struct PostWeightRDB {
    weight: f32,
    timestamp: NaiveDateTime,
    device_id: i32,
}

impl PostWeightRDB {
    fn save(self, conn: &MysqlConnection) -> Result<(), MyError> {
        diesel::insert_into(weights::table)
            .values(self)
            .execute(conn)?;
        Ok(())
    }
}

#[derive(Debug, Queryable, Identifiable, PartialEq, Clone)]
#[table_name = "weights"]
struct FetchWeightRDB {
    id: i32,
    timestamp: NaiveDateTime,
    created_at: NaiveDateTime,
    weight: f32,
    device_id: i32,
}

impl FetchWeightRDB {
    fn fetch_by_datetime(
        conn: &MysqlConnection,
        datetime_from: NaiveDateTime,
        datetime_to: NaiveDateTime,
    ) -> Result<Vec<FetchWeightRDB>, MyError> {
        // let records = sql_query(
        //     "SELECT id, weight, timestamp, device_id, created_at FROM weights WHERE timestamp BETWEEN ? AND ?",
        // ).bind(datetime_from).bind(datetime_to).load::<FetchWeightRDB>(conn)?;
        println!("{}{}", datetime_from, datetime_to);
        let records = weights::dsl::weights
            .filter(timestamp.gt(datetime_from))
            .filter(timestamp.lt(datetime_to))
            .load::<FetchWeightRDB>(conn)?;

        Ok(records)
    }
}

pub fn save(
    conn: &MysqlConnection,
    weight: f32,
    _timestamp: i64,
    device_id: i32,
) -> Result<(), MyError> {
    let naive = NaiveDateTime::from_timestamp_millis(_timestamp * 1000).unwrap();
    println!("naive: {:?}", naive);
    PostWeightRDB {
        weight,
        device_id,
        timestamp: naive,
    }
    .save(conn)?;
    Ok(())
}

pub fn fetch_by_datetime(
    conn: &MysqlConnection,
    datetime_from: CDateTime<Local>,
    datetime_to: CDateTime<Local>,
) -> Result<FetchWeightResponse, MyError> {
    let records = FetchWeightRDB::fetch_by_datetime(
        conn,
        datetime_from.naive_utc(),
        datetime_to.naive_utc(),
    )?;
    println!("{}", Local.from_utc_datetime(&records[0].timestamp));
    let weights = records
        .into_iter()
        .map(|r| Weight {
            id: r.id,
            weight: r.weight,
            timestamp: Local.from_utc_datetime(&r.timestamp),
            device_id: r.device_id,
            created_at: Local.from_utc_datetime(&r.created_at),
        })
        .collect::<Vec<Weight>>();

    Ok(FetchWeightResponse { weights })
}
