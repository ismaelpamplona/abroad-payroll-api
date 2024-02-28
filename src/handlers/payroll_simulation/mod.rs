use crate::{
    handlers::meta_payroll_items::{
        PayrollItemsResponse, TransactionType, SELECT_QUERY as SELECT_PAYROLL_ITEMS_QUERY,
    },
    response::ApiResponse,
};
use axum::{extract::Extension, http::StatusCode, response::IntoResponse, Json};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::env::var;
use uuid::Uuid;

pub mod calc;
pub mod calc_af;
pub mod calc_gets;
pub mod calc_irfe;
pub mod calc_irpf;
pub mod calc_manual_entry;
pub mod calc_rb_or_irex;
pub mod close;
pub mod utils;

pub use calc::calc;
pub use close::close;

#[derive(Deserialize, Serialize, FromRow)]
pub struct CalcPayload {
    pub payroll_date: NaiveDate,
    pub rate: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClosePayload {
    pub simulation_id: Uuid,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReceiptToPay {
    pub receipt_id: Uuid,
    pub value: f64,
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
    has_retention_bonus: bool,
    payroll_brl_pss: f64,
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
        ts.law_date,
        p.cpf,
        p.bank_id,
        b.name as bank_name,
        b.number as bank_number,
        p.bank_agency,
        p.bank_agency_account,
        p.has_retention_bonus,
        p.payroll_brl_pss
    FROM time_served_abroad ts
    JOIN people p ON ts.person_id = p.id
    JOIN roles r ON p.role_id = r.id
    JOIN classes cl ON p.class_id = cl.id
    JOIN banks b ON p.bank_id = b.id
    JOIN cities ci ON ts.city_id = ci.id
    JOIN countries co ON ci.country_id = co.id
    JOIN roles_classes_indexes rci ON p.role_id = rci.role_id AND p.class_id = rci.class_id
    JOIN fc_rf_by_roles rcirf ON p.role_id = rcirf.role_id AND p.class_id = rcirf.class_id
    JOIN fc_rf_by_city cirf ON ts.city_id = cirf.city_id
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
    SELECT
        pr.rf_receipt_id,
        pr.payroll_closed_item_id,
        r.person_id,
        r.start_date,
        r.end_date
    FROM paid_rf_receipts pr
    JOIN rf_payment_receipts r ON pr.rf_receipt_id = r.id
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct ManualEntriesRes {
    person_id: Uuid,
    payroll_item: Uuid,
    value: f64,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

pub const SELECT_MANUAL_ENTRIES_QUERY: &str = "
    SELECT * FROM manual_entries me
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct IncomeTaxesRes {
    from_value: f64,
    to_value: f64,
    tax_rate: f64,
    parcel_deductible_value: f64,
    law: String,
    start_from: NaiveDate,
}

pub const SELECT_INCOME_TAX_TABLE: &str = "
    SELECT *
    FROM public.progressive_income_tax_table
    WHERE start_from = (
        SELECT MAX(start_from)
        FROM public.progressive_income_tax_table
    )
";

pub const WHERE_LIMIT_QUERY: &str = "
    WHERE law_date = (
        SELECT MAX(law_date)
        FROM public.cf_limit_exchange_rate
    )
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct LimitRateRes {
    value: f64,
}

pub const SELECT_LIMIT_RATE_QUERY: &str = "
    SELECT *
    FROM cf_limit_exchange_rate
    WHERE law_date = (
        SELECT MAX(law_date)
        FROM public.cf_limit_exchange_rate
    )
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct LimitValueRes {
    value: f64,
}

pub const SELECT_CF_LIMIT_VALUE_QUERY: &str = "
    SELECT * 
    FROM cf_limit_value
    WHERE law_date = (
        SELECT MAX(law_date)
        FROM public.cf_limit_value
    )
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct PayrollRes {
    id: Uuid,
    payroll_date: NaiveDate,
    payroll_item: Uuid,
    code: String,
    short_name: String,
    description: String,
    transaction_type: TransactionType,
}

pub const SELECT_PAYROLL_SIMULATION_QUERY: &str = "
    SELECT 
        ps.id, 
        ps.date as payroll_date,
        ps.payroll_item,
        i.code,
        i.short_name,
        i.description,
        i.transaction_type,
        ps.person_id,
        p.name as person_name,
        ps.value
    FROM payroll_simulation ps
    JOIN meta_payroll_items i ON ps.payroll_item = i.id
    JOIN people p ON ps.person_id = p.id
";

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct SimulationRes {
    id: Uuid,
    simulation_id: Uuid,
    payroll_item: Uuid,
    person_id: Uuid,
    value: f64,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct ClosedRes {
    id: Uuid,
    closed_id: Uuid,
    payroll_item: Uuid,
    person_id: Uuid,
    value: f64,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct PayrollData {
    payroll_item: Uuid,
    person_id: Uuid,
    value: f64,
    date: NaiveDate,
}

#[derive(Deserialize, Serialize, FromRow, Debug, Clone)]
pub struct PayrollDataWithReceipt {
    payroll_item: Uuid,
    person_id: Uuid,
    value: f64,
    date: NaiveDate,
    receipt_id: Option<Uuid>,
}
