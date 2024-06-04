use async_trait::async_trait;
use bigdecimal::BigDecimal;
use sqlx::Error;
use uuid::Uuid;

use crate::{dtos::route::SaveRouteParamsDTO, models::route::Route};

use super::client::DBClient;

#[async_trait]
pub trait RouteExt {
    async fn get_route(&self, route_id: Option<Uuid>) -> Result<Option<Route>, sqlx::Error>;

    async fn list_routes(&self, page: u32, limit: usize) -> Result<Vec<Route>, sqlx::Error>;

    async fn save_route<B: Into<BigDecimal> + Send, S: Into<String> + Send>(
        &self,
        params: SaveRouteParamsDTO<B, S>,
    ) -> Result<Route, sqlx::Error>;

    async fn delete_route(&self, route_id: Option<Uuid>) -> Result<Option<Route>, sqlx::Error>;
}

#[async_trait]
impl RouteExt for DBClient {
    async fn get_route(&self, route_id: Option<Uuid>) -> Result<Option<Route>, sqlx::Error> {
        if let Some(route_id) = route_id {
            let vehicle = sqlx::query_as!(Route, r#"SELECT * FROM routes WHERE id = $1"#, route_id)
                .fetch_optional(&self.pool)
                .await?;
            return Ok(vehicle);
        }

        Ok(None)
    }

    async fn list_routes(&self, page: u32, limit: usize) -> Result<Vec<Route>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let routes = sqlx::query_as!(
            Route,
            r#"SELECT * FROM routes LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(routes)
    }

    async fn save_route<B: Into<BigDecimal> + Send, S: Into<String> + Send>(
        &self,
        params: SaveRouteParamsDTO<B, S>,
    ) -> Result<Route, sqlx::Error> {
        let SaveRouteParamsDTO {
            initial_lat,
            initial_long,
            final_lat,
            final_long,
            // driver_id,
            initial_address_id,
            final_address_id,
            vehicle_id,
        } = params;

        let initial_address_id = initial_address_id
            .map(|id| Uuid::parse_str(&id.into()))
            .transpose()
            .map_err(|e| {
                Error::Protocol(format!("Failed to parse initial_address_id: {}", e).into())
            })?;

        let final_address_id = final_address_id
            .map(|id| Uuid::parse_str(&id.into()))
            .transpose()
            .map_err(|e| {
                Error::Protocol(format!("Failed to parse final_address_id: {}", e).into())
            })?;

        let vehicle_id = Uuid::parse_str(&vehicle_id.into())
            .map_err(|e| Error::Protocol(format!("Failed to parse vehicle_id: {}", e).into()))?;

        let route = sqlx::query_as!(
            Route,
            r#"
            INSERT INTO routes (initial_lat, initial_long, final_lat, final_long, initial_address_id, final_address_id, vehicle_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7) 
            RETURNING *"#,
            &initial_lat.into(),
            &initial_long.into(),
            &final_lat.map(Into::into).unwrap_or_default(),
            &final_long.map(Into::into).unwrap_or_default(),
            initial_address_id,
            final_address_id,
            vehicle_id,
            // Uuid::parse_str(&driver_id.into()).unwrap(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(route)
    }

    async fn delete_route(&self, route_id: Option<Uuid>) -> Result<Option<Route>, sqlx::Error> {
        let mut route = None;

        if let Some(route_id) = route_id {
            route = sqlx::query_as!(
                Route,
                r#"DELETE FROM routes WHERE id = $1 RETURNING *"#,
                route_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(route)
    }
}
