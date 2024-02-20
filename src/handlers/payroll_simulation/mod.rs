use crate::response::{get_error_status, handle_error, ApiResponse, Meta};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::env::var;
use uuid::Uuid;

pub mod calc;
pub mod formulas;
pub mod utils;

pub use calc::calc;

#[derive(Deserialize, Serialize, FromRow)]
pub struct CalcPayload {
    pub payroll_date: NaiveDate,
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct PeopleRes {
    person_id: Uuid,
    person_name: String,
    role_id: Uuid,
    role_name: String,
    class_id: Uuid,
    class_name: String,
    rci_fc_rb: f64,
    rci_fc_irex: f64,
    rci_fc_irfe: f64,
    country_id: Uuid,
    country_name: String,
    city_id: Uuid,
    city_name: String,
    city_fc_rb: f64,
    city_fc_irex: f64,
    city_fc_irfe: f64,
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
        rcirf.value as rci_fc_irfe,
        ci.country_id,
        co.name as country_name,
        ts.city_id,
        ci.name as city_name,
        ci.fc_rb as city_fc_rb,
        ci.fc_irex as city_fc_irex,
        cirf.value as city_fc_irfe,
        ts.boarding_date,
        ts.start_date,
        ts.end_date,
        ts.law,
        ts.law_date,z
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
    JOIN fc_rf_by_roles rcirf ON p.role_id = rf.role_id AND p.class_id = rf.class_id
    JOIN fc_rf_by_city cirf ON ts.city_id = cirf.id
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct DependentsRes {
    person_id: Uuid,
    birth_date: NaiveDate,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
    ir: bool,
    type_id: Uuid,
    value: f64,
}

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

#[derive(Deserialize, Serialize, FromRow, Debug)]
pub struct ReceiptsRes {
    rf_receipt_id: Uuid,
    person_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
    rate: f64,
    value: f64,
}

pub const SELECT_RF_RECEIPTS_QUERY: &str = "
    SELECT 
        rf.id as rf_receipt_id,
        ts.person_id,
        rf.start_date,
        rf.end_date,
        rf.rate,
        rf.value
    FROM time_served_abroad ts
    JOIN rf_payment_receipts rf ON rf.person_id = ts.person_id
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct TimeServedAbroadRes {
    boarding_date: NaiveDate,
    end_date: Option<NaiveDate>,
}

pub const SELECT_TIME_SERVED_ABROAD_QUERY: &str = "
    SELECT 
        ts.boarding_date,
        ts.end_date
    FROM time_served_abroad ts
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct PaidReceiptsRes {
    rf_receipt_id: Uuid,
    person_id: Uuid,
    start_date: NaiveDate,
    end_date: NaiveDate,
    payroll_closed_item_id: Uuid,
}

pub const SELECT_PAID_RECEIPTS_QUERY: &str = "
    SELECT * 
        pr.rf_receipt_id,
        r.person_id,
        r.start_date,
        r.end_date
    FROM paid_rf_receipts pr
    JOIN rf_payment_receipts r ON pr.rf_receipt_id = r.id;
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct PayrollData {
    payroll_item: Uuid,
    person_id: Uuid,
    value: f64,
    date: NaiveDate,
}
