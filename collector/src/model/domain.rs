use super::super::schema::domain;

#[derive(Queryable)]
pub struct Domain {
    pub rowid: i64,
    pub rank: i32,
    pub fqdn: String,
    pub sub: String,
    pub root: String,
    pub suffix: String,
}

#[derive(Insertable)]
#[table_name = "domain"]
pub struct NewDomain<'a> {
    pub rank: &'a i32,
    pub fqdn: &'a str,
    pub sub: &'a str,
    pub root: &'a str,
    pub suffix: &'a str,
}
