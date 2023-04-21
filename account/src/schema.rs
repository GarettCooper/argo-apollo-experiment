use async_graphql::{EmptyMutation, EmptySubscription, SimpleObject};
use chrono::NaiveDateTime;

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[async_graphql::ComplexObject]
impl Account {
    async fn masked_email(&self) -> String {
        let mut masked = self.email.clone();
        masked.replace_range(
            1..self.email.len() - 1,
            "*".repeat(self.email.len() - 2).as_str(),
        );
        masked
    }

    async fn test(&self) -> String {
        "test".to_string()
    }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn account(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> async_graphql::Result<Account> {
        Ok(Account {
            id,
            name: "John".to_string(),
            email: "test@email.com".to_string(),
            created_at: NaiveDateTime::MIN,
            updated_at: NaiveDateTime::MAX,
        })
    }

    #[graphql(entity)]
    async fn account_by_id(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: i32,
    ) -> async_graphql::Result<Account> {
        Ok(Account {
            id,
            name: "John".to_string(),
            email: "test@email.com".to_string(),
            created_at: NaiveDateTime::MIN,
            updated_at: NaiveDateTime::MAX,
        })
    }
}
