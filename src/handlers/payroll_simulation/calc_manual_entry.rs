// id uuid NOT NULL DEFAULT uuid_generate_v4(),
// person_id uuid NULL,
// payroll_item uuid NULL,
// value float8 NOT NULL,
// start_date date NOT NULL,
// end_date date NOT NULL,
// payroll_item_id uuid NOT NULL UNIQUE,
// created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
// updated_at timestamp NULL,
// e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
// CONSTRAINT manual_entries_pkey PRIMARY KEY (id)

use super::*;
use chrono::Datelike;
use utils::*;

pub fn calc_manual_entry_to_pay(
    entries: Vec<&ManualEntriesRes>,
    payroll_date: NaiveDate,
) -> Vec<PayrollData> {
    entries
        .into_iter()
        .map(|e| PayrollData {
            payroll_item: e.payroll_item,
            person_id: e.person_id,
            value: e.value,
            date: payroll_date,
        })
        .collect()
}
