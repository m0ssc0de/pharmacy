use super::schema::medicines;
#[derive(Serialize, Queryable)]
pub struct Medicine {
    pub id: String,
    pub title: String,
    pub descr: String,
    pub price: i32,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "medicines"]
pub struct NewMedicine<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub price: i32,
    pub descr: &'a str,
}
