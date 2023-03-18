use sea_orm_migration::prelude::*;

use crate::{m20230318_004047_resource_type::ResourceType, m20230318_004236_resource::Resource};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(ResouceResouceType::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ResouceResouceType::ResourceId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("resouce_type_types_resouce_fk")
                            .from(ResouceResouceType::Table, ResouceResouceType::ResourceId)
                            .to(Resource::Table, Resource::Id),
                    )
                    .col(
                        ColumnDef::new(ResouceResouceType::ResourceTypeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("resouce_type_types_resouce_type_fk")
                            .from(
                                ResouceResouceType::Table,
                                ResouceResouceType::ResourceTypeId,
                            )
                            .to(ResourceType::Table, ResourceType::Id),
                    )
                    .primary_key(
                        Index::create()
                            .col(ResouceResouceType::ResourceId)
                            .col(ResouceResouceType::ResourceTypeId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ResouceResouceType::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum ResouceResouceType {
    Table,
    ResourceId,
    ResourceTypeId,
}
