use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

impl Pagination {
    pub fn offset(&self) -> usize {
        self.page.unwrap_or(1).saturating_sub(1) * self.page_size()
    }

    pub fn page_size(&self) -> usize {
        self.page_size.unwrap_or(10).min(100)
    }
}

pub struct Filter<'a> {
    pub name: &'a str,
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

    pub fn error(error: ErrorDetail) -> Self {
        ApiResponse {
            data: None,
            meta: None,
            error: Some(error),
        }
    }
}

pub fn generate_filter_clauses(filters: Vec<Filter>) -> String {
    filters.into_iter().fold(String::new(), |mut acc, filter| {
        if let Some(value) = filter.val {
            value.split(',').for_each(|val| {
                if !acc.is_empty() {
                    acc.push_str(&format!(" {} ", filter.conj));
                }
                acc.push_str(&format!("{} ILIKE '%{}%'", filter.name, val));
            });
        }
        acc
    })
}
