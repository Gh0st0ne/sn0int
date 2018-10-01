use errors::*;
use diesel::prelude::*;
use json::LuaJsonValue;
use models::*;
use ser;
use serde_json;


#[derive(Identifiable, Queryable, Associations, Serialize, PartialEq, Debug)]
#[belongs_to(Subdomain)]
#[table_name="urls"]
pub struct Url {
    pub id: i32,
    pub subdomain_id: i32,
    pub value: String,
    pub status: Option<i32>,
    pub body: Option<Vec<u8>>,
}

impl Model for Url {
    type ID = str;

    fn list(db: &Database) -> Result<Vec<Self>> {
        use schema::urls::dsl::*;

        let results = urls.load::<Self>(db.db())?;

        Ok(results)
    }

    fn filter(db: &Database, filter: &Filter) -> Result<Vec<Self>> {
        use schema::urls::dsl::*;

        let query = urls.filter(filter.sql());
        let results = query.load::<Self>(db.db())?;

        Ok(results)
    }

    fn by_id(db: &Database, my_id: i32) -> Result<Self> {
        use schema::urls::dsl::*;

        let url = urls.filter(id.eq(my_id))
            .first::<Self>(db.db())?;

        Ok(url)
    }

    fn id(db: &Database, query: &Self::ID) -> Result<i32> {
        use schema::urls::dsl::*;

        let url_id = urls.filter(value.eq(query))
            .select(id)
            .first::<i32>(db.db())?;

        Ok(url_id)
    }

    fn id_opt(db: &Database, query: &Self::ID) -> Result<Option<i32>> {
        use schema::urls::dsl::*;

        let url_id = urls.filter(value.eq(query))
            .select(id)
            .first::<i32>(db.db())
            .optional()?;

        Ok(url_id)
    }
}

pub struct PrintableUrl {
    value: String,
    status: Option<u16>,
}

impl fmt::Display for PrintableUrl {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{:?}", self.value)?;

        if let Some(status) = self.status {
            write!(w, " ({})", status)?;
        }

        Ok(())
    }
}

impl Printable<PrintableUrl> for Url {
    fn printable(&self, _db: &Database) -> Result<PrintableUrl> {
        Ok(PrintableUrl {
            value: self.value.to_string(),
            status: self.status.map(|x| x as u16),
        })
    }
}

pub struct DetailedUrl {
    id: i32,
    value: String,
    status: Option<u16>,
}

impl fmt::Display for DetailedUrl {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "\x1b[32m#{}\x1b[0m, \x1b[32m{:?}\x1b[0m", self.id, self.value)?;

        if let Some(status) = self.status {
            write!(w, " (\x1b[33m{}\x1b[0m)", status)?;
        }

        Ok(())
    }
}

impl Detailed for Url {
    type T = DetailedUrl;

    fn detailed(&self, _db: &Database) -> Result<Self::T> {
        Ok(DetailedUrl {
            id: self.id,
            value: self.value.to_string(),
            status: self.status.map(|x| x as u16),
        })
    }
}

#[derive(Insertable)]
#[table_name="urls"]
pub struct NewUrl<'a> {
    pub subdomain_id: i32,
    pub value: &'a str,
    pub status: Option<i32>,
    pub body: Option<&'a [u8]>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="urls"]
pub struct NewUrlOwned {
    pub subdomain_id: i32,
    pub value: String,
    pub status: Option<i32>,
    #[serde(deserialize_with="ser::opt_string_or_bytes")]
    pub body: Option<Vec<u8>>,
}

impl NewUrlOwned {
    pub fn from_lua(x: LuaJsonValue) -> Result<NewUrlOwned> {
        let x = serde_json::from_value(x.into())?;
        Ok(x)
    }
}

impl Printable<PrintableUrl> for NewUrlOwned {
    fn printable(&self, _db: &Database) -> Result<PrintableUrl> {
        Ok(PrintableUrl {
            value: self.value.to_string(),
            status: self.status.map(|x| x as u16),
        })
    }
}