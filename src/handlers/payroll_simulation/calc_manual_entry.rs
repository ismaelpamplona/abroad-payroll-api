use super::*;

pub fn calc_manual_entry_to_pay(
    entries: Vec<&ManualEntriesRes>,
    payroll_date: NaiveDate,
) -> Vec<PayrollDataWithReceipt> {
    entries
        .into_iter()
        .map(|e| PayrollDataWithReceipt {
            payroll_item: e.payroll_item,
            person_id: e.person_id,
            value: e.value,
            date: payroll_date,
            receipt_id: None,
        })
        .collect()
}
