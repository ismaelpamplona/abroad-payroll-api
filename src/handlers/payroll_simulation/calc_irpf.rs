use std::collections::HashMap;

use super::*;
use utils::*;

pub fn calc_irfe(
    payroll_data: Vec<PayrollData>,
    map_item: HashMap<Uuid, (bool, TransactionType)>,
) -> PayrollData {
    for item in &payroll_data {
        let (consider_for_ir, transaction_type) = map_item.get(&item.payroll_item).unwrap();
        let mut gross_value = 0.0;
        if *consider_for_ir && matches!(transaction_type, TransactionType::Credit) {
            gross_value += item.value;
        }
        let is_at = item.payroll_item == Uuid::parse_str(&var("ID_AT").unwrap()).unwrap();
        if is_at {
            gross_value -= item.value;
        }
        let mut taxable_value = gross_value * 0.25;
        if *consider_for_ir && matches!(transaction_type, TransactionType::Debit) && !is_at {
            taxable_value -= item.value;
        }
    }
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_calc_irpf() {
        //
    }
}
