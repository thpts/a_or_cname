use super::super::schema::record;

#[derive(Queryable)]
pub struct Record {
    pub rowid: i64,
    pub domain: i64,
    pub is_www: bool,
    pub parent: Option<i64>,
    pub response_code: i32,
    pub record_type: Option<String>,
    pub ttl: Option<i32>,
    pub address: Option<String>,
    pub asn: Option<i32>,
    pub query_time: i64,
}

#[derive(Insertable)]
#[table_name = "record"]
pub struct NewRecord<'a> {
    pub domain: &'a i64,
    pub parent: Option<&'a i64>,
    pub is_www: &'a bool,
    pub response_code: &'a i32,
    pub record_type: Option<&'a str>,
    pub ttl: Option<&'a i32>,
    pub address: Option<&'a str>,
    pub asn: Option<&'a i32>,
    pub query_time: &'a i64,
}
