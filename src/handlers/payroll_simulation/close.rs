use super::*;
use crate::response::{get_error_status, handle_error, ErrorDetail, SuccessInsert};

pub async fn close(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ClosePayload>,
) -> impl IntoResponse {
    let query_simulation = format!(
        "SELECT * FROM public.payroll_simulation WHERE simulation_id = {}",
        payload.simulation_id
    );

    let result_simulation = sqlx::query_as::<_, SimulationRes>(&query_simulation)
        .fetch_all(&pool)
        .await;
    let closed_id = Uuid::new_v4();

    todo!()
}
