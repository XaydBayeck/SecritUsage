use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Model {
    id: u32,
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Clone)]
pub enum Selector {
    Id(u32),
    Name(String),
    Email(String),
}

#[derive(Debug, Clone)]
pub enum FixSelector {
    Name(String),
    Email(String),
    Password(String),
}

#[derive(Debug, Clone, Deserialize)]
pub enum Login {
    Name(String),
    Email(String),
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLogin {
    pub login: Login,
    pub password: String,
}

impl From<Selector> for (&str, String) {
    fn from(value: Selector) -> Self {
        use Selector::*;

        match value {
            Id(id) => ("id", id.to_string()),
            Name(name) => ("name", name),
            Email(email) => ("email", email),
        }
    }
}

impl Model {
    pub async fn select_by(selector: Selector, pool: &SqlitePool) -> Result<Self, sqlx::Error> {
        let (query, cond) = selector.into();

        let query = format!("select * from users where {query} = ?");

        sqlx::query_as(&query).bind(cond).fetch_one(pool).await
    }

    pub async fn delete_by(selector: Selector, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let (query, cond) = selector.into();

        let query = format!("delete from users where {query} = ?");

        sqlx::query(&query).bind(cond).fetch_all(pool).await?;

        Ok(())
    }

    pub async fn delete(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(r"delete from users where id = ? and name = ? and email = ? and password = ?")
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.email)
            .bind(&self.password)
            .fetch_all(pool)
            .await?;

        Ok(())
    }

    pub async fn update(
        &mut self,
        selector: FixSelector,
        pool: &SqlitePool,
    ) -> Result<(), sqlx::Error> {
        use FixSelector::*;

        let (query, set) = match selector {
            Name(name) => {
                self.name = name.clone();
                ("name", name)
            }
            Email(email) => {
                self.email = email.clone();
                ("email", email)
            }
            Password(password) => {
                self.password = password.clone();
                ("password", password)
            }
        };

        let query = format!("update users set {query} = $1 where id = $2");

        sqlx::query(&query)
            .bind(set)
            .bind(&self.id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

impl User {
    pub async fn insert(&self, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r"
        insert into users (name, email, password) 
        values ($1, $2, $3)",
        )
        .bind(&self.name)
        .bind(&self.email)
        .bind(&self.password)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(&self, selector: Selector, pool: &SqlitePool) -> Result<(), sqlx::Error> {
        let (query, cond) = selector.into();

        let query = format!(
            r"
        update users 
        set name = $1, email = $2, password = $3
        where {query} = ?
        "
        );

        sqlx::query(&query)
            .bind(&self.name)
            .bind(&self.email)
            .bind(&self.password)
            .bind(cond)
            .execute(pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use sqlx::SqlitePool;

    use crate::user::{FixSelector, Model, Selector};

    use super::User;

    #[tokio::test]
    async fn insert() {
        let pool = SqlitePool::connect("sqlite://test.db").await;

        assert!(pool.is_ok());

        let pool = pool.unwrap();

        let user = User {
            name: "Alen".into(),
            email: "example@xxx.com".into(),
            password: "123456".into(),
        };

        assert!(user.insert(&pool).await.is_ok());

        let model = Model::select_by(Selector::Name("Alen".into()), &pool).await;

        assert!(model.is_ok());

        let mut model = model.unwrap();

        assert_eq!(user.name, model.name);
        assert_eq!(user.email, model.email);
        assert_eq!(user.password, model.password);

        assert!(model
            .update(FixSelector::Email("example@qq.cn.com".to_string()), &pool)
            .await
            .is_ok());

        assert_eq!(model.email, "example@qq.cn.com");

        let new_model = Model::select_by(Selector::Id(model.id), &pool).await;

        assert!(new_model.is_ok());

        let new_model = new_model.unwrap();

        assert_eq!(model.email, new_model.email);

        let model = new_model;

        assert!(model.delete(&pool).await.is_ok());
    }
}
