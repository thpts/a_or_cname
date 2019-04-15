use super::super::schema::record;

#[derive(Queryable)]
pub struct Record {
    pub domain: i32,
    pub parent: i32,
    pub response_code: i32,
    pub record_type: String,
    pub ttl: i32,
    pub address: String,
    pub asn: i32,
}

#[derive(Insertable)]
#[table_name = "record"]
pub struct NewRecord<'a> {
    pub domain: &'a i32,
    pub parent: &'a i32,
    pub response_code: &'a i32,
    pub record_type: &'a str,
    pub ttl: &'a i32,
    pub address: &'a str,
    pub asn: &'a i32,
}
