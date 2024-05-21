use async_trait::async_trait;
use uuid::Uuid;

use super::client::DBClient;
use crate::models::collaborator::Collaborator;

#[async_trait]
pub trait CollaboratorExt {
    async fn get_collaborator(
        &self,
        collaborator_id: Option<Uuid>,
        cpf: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<Collaborator>, sqlx::Error>;

    async fn list_collaborators(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<Collaborator>, sqlx::Error>;

    async fn save_collaborator<T: Into<String> + Send>(
        &self,
        name: T,
        cpf: T,
        rg: T,
        email: T,
    ) -> Result<Collaborator, sqlx::Error>;

    async fn delete_collaborator(
        &self,
        collaborator_id: Option<Uuid>,
    ) -> Result<Option<Collaborator>, sqlx::Error>;
}

#[async_trait]
impl CollaboratorExt for DBClient {
    async fn get_collaborator(
        &self,
        collaborator_id: Option<Uuid>,
        cpf: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<Collaborator>, sqlx::Error> {
        let mut collaborator: Option<Collaborator> = None;

        if let Some(collaborator_id) = collaborator_id {
            collaborator = sqlx::query_as!(
                Collaborator,
                r#"SELECT * FROM collaborators WHERE id = $1"#,
                collaborator_id
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(cpf) = cpf {
            collaborator = sqlx::query_as!(
                Collaborator,
                r#"SELECT * FROM collaborators WHERE cpf = $1"#,
                cpf
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(email) = email {
            collaborator = sqlx::query_as!(
                Collaborator,
                r#"SELECT * FROM collaborators WHERE email = $1"#,
                email
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(collaborator)
    }

    async fn list_collaborators(
        &self,
        page: u32,
        limit: usize,
    ) -> Result<Vec<Collaborator>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let collaborators = sqlx::query_as!(
            Collaborator,
            r#"SELECT id, name, cpf, rg, email, created_at, updated_at FROM collaborators LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(collaborators)
    }

    async fn save_collaborator<T: Into<String> + Send>(
        &self,
        name: T,
        cpf: T,
        rg: T,
        email: T,
    ) -> Result<Collaborator, sqlx::Error> {
        let collaborator = sqlx::query_as!(
            Collaborator,
            r#"INSERT INTO collaborators (name, cpf, rg, email) VALUES ($1, $2, $3, $4) RETURNING *"#,
            &name.into(),
            &cpf.into(),
            &rg.into(),
            &email.into(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(collaborator)
    }

    async fn delete_collaborator(
        &self,
        collaborator_id: Option<Uuid>,
    ) -> Result<Option<Collaborator>, sqlx::Error> {
        let mut collaborator = None;

        if let Some(collaborator_id) = collaborator_id {
            collaborator = sqlx::query_as!(
                Collaborator,
                r#"DELETE FROM collaborators WHERE id = $1 RETURNING *"#,
                collaborator_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(collaborator)
    }
}
