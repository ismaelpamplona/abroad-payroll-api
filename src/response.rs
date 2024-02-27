use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::error::Error;
use std::convert::TryInto;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub meta: Option<Meta>,
    pub error: Option<ErrorDetail>,
}

#[derive(Serialize)]
pub struct Meta {
    pub total_count: Option<usize>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct SuccessDelete {
    id: Uuid,
    message: String,
}

#[derive(Serialize)]
pub struct SuccessInsert {
    message: String,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
    pub limit: Option<usize>,
}

impl Pagination {
    pub fn offset(&self, total_count: usize) -> usize {
        self.page.unwrap_or(1).saturating_sub(1) * self.page_size(total_count)
    }

    pub fn page_size(&self, total_count: usize) -> usize {
        let limit = self.limit.unwrap_or(10).min(100);
        if total_count < limit {
            return total_count;
        }
        self.page_size.unwrap_or(limit).min(100)
    }
}

#[derive(Debug)]
pub enum Operator {
    Equal,
    ILIKE,
}

#[derive(Debug)]
pub struct Filter<'a> {
    pub name: &'a str,
    pub op: Operator,
    pub val: Option<&'a String>,
    pub conj: &'a str,
}

impl<T> ApiResponse<T> {
    pub fn success_list(data: T, meta: Meta) -> Self {
        ApiResponse {
            data: Some(data),
            meta: Some(meta),
            error: None,
        }
    }

    pub fn success_one(data: T) -> Self {
        ApiResponse {
            data: Some(data),
            meta: None,
            error: None,
        }
    }

    pub fn success_delete(id: Uuid) -> Json<SuccessDelete> {
        let response = SuccessDelete {
            id,
            message: format!("Resource has been successfully deleted."),
        };
        Json(response)
    }

    pub fn success_insert() -> Json<SuccessInsert> {
        let response = SuccessInsert {
            message: format!("Resource has been successfully inserted."),
        };
        Json(response)
    }

    pub fn error(error: ErrorDetail) -> Self {
        ApiResponse {
            data: None,
            meta: None,
            error: Some(error),
        }
    }
}

pub fn generate_filter_clauses(filters: Vec<Filter>) -> String {
    let conditions: Vec<String> = filters
        .into_iter()
        .filter_map(|filter| {
            filter.val.as_ref().map(|value| {
                let value_parts = value.split(',').map(|val| match filter.op {
                    Operator::ILIKE => {
                        format!("unaccent({}) ILIKE unaccent('%{}%')", filter.name, val)
                    }
                    Operator::Equal => format!("{} = '{}'", filter.name, val),
                });
                value_parts
                    .collect::<Vec<String>>()
                    .join(&format!(" {} ", filter.conj))
            })
        })
        .collect();

    if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    }
}

pub fn get_pg_error_code(error: &Error) -> Option<String> {
    match error {
        sqlx::error::Error::Database(database_error) => {
            if let Some(code) = database_error.code() {
                return Some(code.to_string());
            }
        }
        _ => {}
    }
    None
}

pub fn handle_error(error: &Error) -> ErrorDetail {
    println!("{:?}", error);
    let (status_code, message) = match get_pg_error_code(&error) {
        Some(code) => match code.as_str() {
            "23505" => (StatusCode::CONFLICT, "This data already exists"),
            "23503" => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Cannot delete/update because it is still linked to other related data.",
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        },
        None => (StatusCode::NOT_FOUND, "Not found"),
    };

    ErrorDetail {
        code: status_code.as_u16(),
        message: message.to_string(),
    }
}

pub fn get_error_status(error: &Error) -> StatusCode {
    handle_error(error)
        .code
        .try_into()
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}
