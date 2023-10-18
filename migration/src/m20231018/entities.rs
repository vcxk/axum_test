use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Account,
    PassHash,
    Email,
    Phone,
    Sex,
    BirthDay,
    CreateTime,
}

#[derive(DeriveIden)]
pub enum LoginToken {
    Table,
    Id,
    UserId,
    Time,
    Token,
    Ip,
    Device,
}

#[derive(DeriveIden)]
pub enum UserRole {
    Table,
    Id,
    UserId,
    RoleId,
    CreateTime,
    CreatorId
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    Name,
    Desc
}

#[derive(DeriveIden)]
pub enum Permit {
    Table,
    Id,
    Name,
    Desc,
}

#[derive(DeriveIden)]
pub enum  RolePermit {
    Table,
    Id,
    RoleId,
    PermitId
}
