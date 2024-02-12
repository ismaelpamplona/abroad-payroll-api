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
FROM public.time_served_abroad ts
JOIN people p ON ts.person_id = p.id
JOIN roles r ON p.role_id = r.id
JOIN classes cl ON p.class_id = cl.id
JOIN banks b ON p.bank_id = b.id
JOIN cities ci ON ts.city_id = ci.id
JOIN countries co ON ci.country_id = co.id
JOIN roles_classes_indexes rci ON p.role_id = rci.role_id AND p.class_id = rci.class_id
WHERE 
    ts.end_date IS NULL
    OR (
        ts.end_date >= DATE_TRUNC('month', CURRENT_DATE) 
        AND ts.end_date <= DATE_TRUNC('month', CURRENT_DATE) + INTERVAL '1 month - 1 day'
    );