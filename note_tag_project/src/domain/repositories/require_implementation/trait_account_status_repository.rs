use sea_orm_migration::async_trait;



#[async_trait::async_trait]
pub trait AccountStatusRepository {
    async fn create(&self) -> Result<(), String>;
    async fn read(&self) -> Result<(), String>;
    async fn read_all(&self) -> Result<(), String>;
    async fn update(&self) -> Result<(), String>;
    async fn delete(&self) -> Result<(), String>;
}