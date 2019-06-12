use super::super::schema::domain;

#[derive(Queryable)]
pub struct Domain {
    pub rowid: i64,
    pub rank: i32,
    pub fqdn: String,
    pub sub: Option<String>,
    pub root: Option<String>,
    pub suffix: Option<String>,
}

#[derive(Insertable)]
#[table_name = "domain"]
pub struct NewDomain<'a> {
    pub rank: &'a i32,
    pub fqdn: &'a str,
    pub sub: Option<&'a str>,
    pub root: Option<&'a str>,
    pub suffix: Option<&'a str>,
}
