use std::collections::HashMap;

use super::*;

pub fn calc_irpf(
    person_payroll_data: &Vec<PayrollDataWithReceipt>,
    map_item: &HashMap<Uuid, (bool, TransactionType)>,
    income_taxes: &Vec<IncomeTaxesRes>,
    payroll_date: NaiveDate,
    rate: f64,
    person_id: Uuid,
) -> PayrollDataWithReceipt {
    let mut gross_value = 0.0;
    let id_at = Uuid::parse_str(&var("ID_AT").unwrap()).unwrap();
    let mut consider_for_ir_debits = 0.0;
    for item in person_payroll_data {
        let (consider_for_ir, transaction_type) = map_item.get(&item.payroll_item).unwrap();
        let is_at = item.payroll_item == id_at;
        if *consider_for_ir && matches!(transaction_type, TransactionType::Credit) && !is_at {
            gross_value += item.value
        }
        if is_at {
            gross_value -= item.value
        }
        if *consider_for_ir && matches!(transaction_type, TransactionType::Debit) && !is_at {
            consider_for_ir_debits += item.value;
        }
    }
    let usd_taxable_value = gross_value * 0.25 - consider_for_ir_debits;

    let brl_taxable_value = usd_taxable_value * rate;

    let filtered_taxes: Vec<&IncomeTaxesRes> = income_taxes
        .iter()
        .filter(|tax| tax.from_value <= brl_taxable_value && brl_taxable_value <= tax.to_value)
        .collect();
    let range = filtered_taxes[0];
    let irpf_brl_value = range.tax_rate * brl_taxable_value - range.parcel_deductible_value;
    PayrollDataWithReceipt {
        payroll_item: Uuid::parse_str(&var("ID_IRPF").unwrap()).unwrap(),
        person_id,
        value: ((irpf_brl_value / rate * 100.0) + 0.5).floor() / 100.0,
        date: payroll_date,
        receipt_id: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_calc_irpf() {
        todo!()
    }
}
