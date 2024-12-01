use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, ValueRef};
use rusqlite::Result as SqlResult;

#[derive(Debug, Clone)]
pub enum Status {
    Lido,
    NaoLido,
    EmProgresso,
}

impl ToSql for Status {
    fn to_sql(&self) -> SqlResult<ToSqlOutput> {
        match self {
            Status::Lido => Ok(ToSqlOutput::from("Lido")),
            Status::NaoLido => Ok(ToSqlOutput::from("Não Lido")),
            Status::EmProgresso => Ok(ToSqlOutput::from("Em Progresso")),
        }
    }
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value.as_str()? {
            "Lido" => Ok(Status::Lido),
            "Não Lido" => Ok(Status::NaoLido),
            "Em Progresso" => Ok(Status::EmProgresso),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
