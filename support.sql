SELECT 
    ts.person_id,
    p.name as person_name,
    p.role_id,
    r.name as role_name,
    p.class_id,
    rci.fc_rb as rci_fc_rb,
    rci.fc_irex as rci_fc_irex,
    cl.name as class_name,
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
WHERE 
    (ts.end_date IS NULL
    OR (ts.end_date >= DATE_TRUNC('month', CURRENT_DATE) -- first day 
        AND ts.end_date <= DATE_TRUNC('month', CURRENT_DATE) + INTERVAL '1 month - 1 day')); -- last day

--        

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
WHERE 
    (ts.end_date IS NULL
    OR (ts.end_date >= DATE_TRUNC('month', CURRENT_DATE) -- first day 
        AND ts.end_date <= DATE_TRUNC('month', CURRENT_DATE) + INTERVAL '1 month - 1 day')); -- last day        

--

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
WHERE 
    (ts.end_date IS NULL
    OR (ts.end_date >= DATE_TRUNC('month', CURRENT_DATE) -- first day 
        AND ts.end_date <= DATE_TRUNC('month', CURRENT_DATE) + INTERVAL '1 month - 1 day'))-- last day        
    AND rf.its_paid = FALSE;




