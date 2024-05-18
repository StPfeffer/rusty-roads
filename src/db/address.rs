use async_trait::async_trait;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::{dtos::address::SaveAddressParamsDTO, models::address::Address};

use super::client::DBClient;

#[async_trait]
pub trait AddressExt {
    async fn get_address(
        &self,
        address_id: Option<Uuid>,
        latitude: Option<BigDecimal>,
        longitude: Option<BigDecimal>,
    ) -> Result<Option<Address>, sqlx::Error>;

    async fn list_addresses(&self, page: u32, limit: usize) -> Result<Vec<Address>, sqlx::Error>;

    async fn save_address<T: Into<String> + Send, B: Into<BigDecimal> + Send>(
        &self,
        params: SaveAddressParamsDTO<T, B>,
    ) -> Result<Address, sqlx::Error>;

    async fn delete_address(
        &self,
        address_id: Option<Uuid>,
    ) -> Result<Option<Address>, sqlx::Error>;
}

#[async_trait]
impl AddressExt for DBClient {
    async fn get_address(
        &self,
        address_id: Option<Uuid>,
        latitude: Option<BigDecimal>,
        longitude: Option<BigDecimal>,
    ) -> Result<Option<Address>, sqlx::Error> {
        let mut address: Option<Address> = None;

        if let Some(address_id) = address_id {
            address = sqlx::query_as!(
                Address,
                r#"SELECT * FROM addresses WHERE id = $1"#,
                address_id
            )
            .fetch_optional(&self.pool)
            .await?;
        } else if let Some(latitude) = latitude {
            if let Some(longitude) = longitude {
                address = sqlx::query_as!(
                    Address,
                    r#"SELECT * FROM addresses WHERE latitude = $1 AND longitude = $2"#,
                    latitude,
                    longitude
                )
                .fetch_optional(&self.pool)
                .await?;
            }
        }

        Ok(address)
    }

    async fn list_addresses(&self, page: u32, limit: usize) -> Result<Vec<Address>, sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let addresses = sqlx::query_as!(
            Address,
            r#"SELECT * FROM addresses LIMIT $1 OFFSET $2"#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(addresses)
    }

    async fn save_address<T: Into<String> + Send, B: Into<BigDecimal> + Send>(
        &self,
        params: SaveAddressParamsDTO<T, B>,
    ) -> Result<Address, sqlx::Error> {
        let SaveAddressParamsDTO {
            address,
            number,
            neighbourhood,
            reference,
            complement,
            zip_code,
            latitude,
            longitude,
            city_id,
        } = params;

        let address = sqlx::query_as!(
            Address,
            r#"
            INSERT INTO addresses (address, number, neighbourhood, reference, complement, zip_code, latitude, longitude, city_id) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
            RETURNING *"#,
            &address.into(),
            &number.into(),
            &neighbourhood.into(),
            &reference.map(Into::into) as _,
            &complement.map(Into::into) as _,
            &zip_code.into(),
            &latitude.map(Into::into).unwrap_or_default(),
            &longitude.map(Into::into).unwrap_or_default(),
            Uuid::parse_str(&city_id.into()).unwrap(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(address)
    }

    async fn delete_address(
        &self,
        address_id: Option<Uuid>,
    ) -> Result<Option<Address>, sqlx::Error> {
        let mut address = None;

        if let Some(address_id) = address_id {
            address = sqlx::query_as!(
                Address,
                r#"DELETE FROM addresses WHERE id = $1 RETURNING *"#,
                address_id
            )
            .fetch_optional(&self.pool)
            .await?;
        }

        Ok(address)
    }
}
