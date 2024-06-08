use async_trait::async_trait;
use uuid::Uuid;

use crate::models::state::State;

use super::client::DBClient;

#[async_trait]
pub trait StateExt {
    async fn get_state(
        &self,
        state_id: Option<Uuid>,
        name: Option<&str>,
        code: Option<&str>,
    ) -> Result<Option<State>, sqlx::Error>;

    async fn list_states(&self, page: u32, limit: usize) -> Result<Vec<State>, sqlx::Error>;

    async fn save_state<T: Into<String> + Send>(
        &self,
        name: T,
        code: T,
        country_id: T,
    ) -> Result<State, sqlx::Error>;

    async fn delete_state(&self, state_id: Option<Uuid>) -> Result<Option<State>, sqlx::Error>;
}

#[async_trait]
impl StateExt for DBClient {
    async fn get_state(
        &self,
        state_id: Option<Uuid>,
        name: Option<&str>,
        code: Option<&str>,
    ) -> Result<Option<State>, sqlx::Error> {
        let mut state: Option<State> = None;

        if let Some(state_id) = state_id {
            state = sqlx::query_as!(State, r#"SELECT * FROM states WHERE id = $1"#, state_id)
                .fetch_optional(&self.pool)
                .await?;
        } else if let Some(name) = name {
            state = sqlx::query_as!(State, r#"SELECT * FROM states WHERE name = $1"#, name)
                .fetch_optional(&self.pool)
                .await?;
        } else if let Some(code) = code {
            state = sqlx::query_as!(State, r#"SELECT * FROM states WHERE code = $1"#, code)
                .fetch_optional(&self.pool)
                .await?;
        }

        Ok(state)
    }

    async fn list_states(&self, page: u32, limit: usize) -> Result<Vec<State>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let states = sqlx::query_as!(
            State,
            r#"SELECT * FROM states LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(states)
    }

    async fn save_state<T: Into<String> + Send>(
        &self,
        name: T,
        code: T,
        country_id: T,
    ) -> Result<State, sqlx::Error> {
        let state = sqlx::query_as!(
            State,
            r#"INSERT INTO states (name, code, country_id) VALUES ($1, $2, $3) RETURNING *"#,
            &name.into(),
            &code.into(),
            Uuid::parse_str(&country_id.into()).unwrap(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(state)
    }

    async fn delete_state(&self, state_id: Option<Uuid>) -> Result<Option<State>, sqlx::Error> {
        let mut state = None;

        if let Some(state_id) = state_id {
            state = sqlx::query_as!(
                State,
                r#"DELETE FROM states WHERE id = $1 RETURNING *"#,
                state_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(state)
    }
}
