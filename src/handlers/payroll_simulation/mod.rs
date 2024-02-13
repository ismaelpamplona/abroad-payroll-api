use crate::response::{get_error_status, handle_error, ApiResponse, Meta};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
pub mod calc;

pub use calc::calc;

#[derive(Deserialize, Serialize, FromRow)]
pub struct CalcPayload {
    pub payroll_date: NaiveDate,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct SelectPeopleResponse {
    person_id: Uuid,
    person_name: String,
    role_id: Uuid,
    role_name: String,
    class_id: Uuid,
    class_name: String,
    rci_fc_rb: f64,
    rci_fc_irex: f64,
    country_id: Uuid,
    country_name: String,
    city_id: Uuid,
    city_name: String,
    boarding_date: NaiveDate,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    law: String,
    law_date: NaiveDate,
    cpf: String,
    bank_id: Uuid,
    bank_name: String,
    bank_number: String,
    bank_agency: String,
    bank_agency_account: String,
}

pub const SELECT_PEOPLE_PAYROLL_QUERY: &str = "
    SELECT 
        ts.person_id,
        p.name as person_name,
        p.role_id,
        r.name as role_name,
        p.class_id,
        cl.name as class_name,
        rci.fc_rb as rci_fc_rb,
        rci.fc_irex as rci_fc_irex,
        ci.country_id,
        co.name as country_name,
        ts.city_id,
        ci.name as city_name,
        ts.boarding_date,
        ts.start_date,
        ts.end_date,
        ts.law,
        ts.law_date,
        p.cpf,
        p.bank_id,
        b.name as bank_name,
        b.number as bank_number,
        p.bank_agency,
        p.bank_agency_account
    FROM time_served_abroad ts
    JOIN people p ON ts.person_id = p.id
    JOIN roles r ON p.role_id = r.id
    JOIN classes cl ON p.class_id = cl.id
    JOIN banks b ON p.bank_id = b.id
    JOIN cities ci ON ts.city_id = ci.id
    JOIN countries co ON ci.country_id = co.id
    JOIN roles_classes_indexes rci ON p.role_id = rci.role_id AND p.class_id = rci.class_id
";

pub const SELECT_DEPENDENTS_QUERY: &str = "
    SELECT 
        ts.person_id,
        p.name as person_name,
        d.name as dependent_name,
        d.birth_date,
        d.start_date,
        d.end_date,
        d.ir,
        d.type_id,
        dt.name as type_name,
        dt.value
    FROM time_served_abroad ts
    JOIN people p ON ts.person_id = p.id
    JOIN dependents d ON ts.person_id = d.person_id
    JOIN dependents_types dt ON dt.id = d.type_id       
";

pub const SELECT_RF_RECEIPTS_QUERY: &str = "
    SELECT 
        ts.person_id,
        p.name as person_name,
        rf.start_date,
        rf.end_date,
        rf.rate,
        rf.value
    FROM time_served_abroad ts
    JOIN people p ON ts.person_id = p.id
    JOIN rf_payment_receipts rf ON rf.person_id = ts.person_id
";

// AND rf.its_paid = FALSE
