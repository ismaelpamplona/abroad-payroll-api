use super::*;

pub async fn calc(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CalcPayload>,
) -> impl IntoResponse {
    let where_clause = format!(
        "WHERE 
            (ts.end_date IS NULL
            OR (ts.end_date >= DATE_TRUNC('month', {})
                AND ts.end_date <= DATE_TRUNC('month', {}) + INTERVAL '1 month - 1 day'))",
        "CURRENT_DATE", "CURRENT_DATE"
    );
    let select_people_data_query = format!("{} {};", SELECT_PEOPLE_PAYROLL_QUERY, where_clause);
    let select_dependents_query = format!("{} {};", SELECT_DEPENDENTS_QUERY, where_clause);
    let select_receits_query = format!(
        "{} {} {};",
        SELECT_RF_RECEIPTS_QUERY, where_clause, "AND rf.its_paid = FALSE"
    );
    let result = sqlx::query_as::<_, SelectPeopleResponse>(&select_people_data_query)
        .bind(&payload.payroll_date)
        .fetch_all(&pool)
        .await;
    println!("{:?}", result);
    todo!()
}
