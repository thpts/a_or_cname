use super::super::schema::record;

#[derive(Queryable)]
pub struct Record {
    pub rowid: i64,
    pub domain: i64,
    pub parent: i64,
    pub response_code: i32,
    pub record_type: String,
    pub ttl: i32,
    pub address: String,
    pub asn: i32,
    pub query_time: i64,
}

#[derive(Insertable)]
#[table_name = "record"]
pub struct NewRecord<'a> {
    pub domain: &'a i64,
    pub parent: &'a i64,
    pub response_code: &'a i32,
    pub record_type: &'a str,
    pub ttl: &'a i32,
    pub address: &'a str,
    pub asn: &'a i32,
    pub query_time: &'a i64,
}
