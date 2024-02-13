INSERT INTO public.roles (id, "name")
VALUES
    ('48b81286-a5aa-493e-99a7-222d464ecf2e', 'Oficial de Inteligência'),
    ('04caa7e1-0957-4e16-8af6-37410f45a016', 'Oficial Técnico de Inteligência'),
    ('b3f82163-fca4-45c5-8e1c-fc7b2f6e417d', 'Agente de Inteligência'),
    ('1c3d2e68-ba1f-4e4d-a7eb-d460d800b46a', 'Agente técnico de inteligência');


--

INSERT INTO public.classes (id, "name")
VALUES
    ('c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', 'Classe Especial'),
    ('2c13d59c-fa5e-44a1-9abf-e92ac39c01b9', 'Primeira Classe'),
    ('7c6cc408-570a-42b5-a693-12284c5e94b5', 'Segunda Classe');


--    

INSERT INTO roles_classes_indexes (id, role_id, class_id, fc_rb, fc_irex)
VALUES
    ('f82c4ab0-51de-4dd5-8774-bb9efeb9b7d3', '48b81286-a5aa-493e-99a7-222d464ecf2e', 'c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', 94, 80),
    ('c6f3b03e-7a0c-4e19-b74b-3b0dfb919e53', '48b81286-a5aa-493e-99a7-222d464ecf2e', '2c13d59c-fa5e-44a1-9abf-e92ac39c01b9', 88, 80),
    ('1e1cb8df-1dd4-472d-bb0a-2d1ab4d28bbf', 'b3f82163-fca4-45c5-8e1c-fc7b2f6e417d', 'c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', 55, 40),
    ('af83e19c-7a50-4840-9f20-0be3e6e94031', 'b3f82163-fca4-45c5-8e1c-fc7b2f6e417d', '2c13d59c-fa5e-44a1-9abf-e92ac39c01b9', 40, 35);

--    

INSERT INTO banks (id, name, number)
VALUES
    ('8e3639d1-0e04-4f98-9d6d-09a1b04f5369', 'Bradesco', '237'),
    ('12c8245d-1c63-4e03-8f2c-6c5979e6a3a4', 'Banco do Brasil', '001'),
    ('e6e5a36e-62f9-46f5-9d65-4974c438e1c1', 'Banco XP', '102');

--    

INSERT INTO countries (id, name)
VALUES
    ('e8c2c998-0e19-482a-b346-f5f6a47c8827', 'Japão'),
    ('1d037c29-c042-4857-9d9b-45fb61b6a4c3', 'Espanha'),
    ('92fdd45c-8ea7-4391-97a7-c0629478482e', 'Peru'),
    ('dda48e8d-0b7f-4fc9-9134-e09ab475ebf3', 'Jordânia'),
    ('a6f4c1a3-1da1-4e6c-a1f5-20893ee2e8d7', 'Estados Unidos'),
    ('4e7e8304-25ef-43ce-a35d-4a184be5ddc2', 'Bolívia');

--

INSERT INTO cities (id, name, country_id, latitude, longitude, fc_rb, fc_irex)
VALUES
    ('cdc50f66-0591-4c2f-a58e-8cb2aa3ce415', 'Tóquio', 'e8c2c998-0e19-482a-b346-f5f6a47c8827', 35.652832, 139.839478, 108.94, 108.94),
    ('f8efc2a0-62c3-49cd-bc7b-4088f9b59c68', 'Madrid', '1d037c29-c042-4857-9d9b-45fb61b6a4c3', 40.416775, -3.70379, 93.6, 64.8),
    ('f7e232b3-700b-4376-8f2e-4aa9275a1016', 'Washington DC', 'a6f4c1a3-1da1-4e6c-a1f5-20893ee2e8d7', 47.751076, -120.740135, 76.7, 76.7);

--

INSERT INTO people (id, name, role_id, class_id, cpf, bank_id, bank_agency, bank_agency_account)
VALUES
    ('a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 'Ana Silva', '48b81286-a5aa-493e-99a7-222d464ecf2e', 'c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', '01234567890', '12c8245d-1c63-4e03-8f2c-6c5979e6a3a4', '0123', '0123456-1'),
    ('6628b9b5-cf0f-492d-834b-220c7aeb2b8c', 'Carlos Santos', 'b3f82163-fca4-45c5-8e1c-fc7b2f6e417d', 'c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', '09876543210', '8e3639d1-0e04-4f98-9d6d-09a1b04f5369', '001', '9999-0'),
    ('03a46f8a-028d-4b41-9d07-7c01612ecfd3', 'Camila Oliveira', '48b81286-a5aa-493e-99a7-222d464ecf2e', '2c13d59c-fa5e-44a1-9abf-e92ac39c01b9', '11111322223', '8e3639d1-0e04-4f98-9d6d-09a1b04f5369', '999', '11111-9');

--

INSERT INTO dependents_types (id, name, value)
VALUES
    ('72c5f0ac-3510-4a02-93cc-812f8b4991ce', 'Esposa', 0.1),
    ('e02c9988-2146-4c59-81e3-1b356f44b9c1', 'Filha solteira, que não receba remuneração', 0.05);

--

INSERT INTO dependents (id, name, person_id, birth_date, start_date, end_date, type_id, ir)
VALUES
    ('2bf18dd3-370c-4f9b-8793-17e76c5f3d0a', 'Maria Oliveira', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2020-9-22', '2023-12-22', NULL, '72c5f0ac-3510-4a02-93cc-812f8b4991ce', FALSE),
    ('d51fbb2b-8815-4a90-82a2-34a963f9f8e7', 'Luiza Silva', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2017-10-1', '2023-12-22', NULL, 'e02c9988-2146-4c59-81e3-1b356f44b9c1', TRUE),
    ('f8f3aa4b-6629-4f43-804f-4d9a56248767', 'Lucas Santos', '6628b9b5-cf0f-492d-834b-220c7aeb2b8c', '2010-10-7', '2023-2-15', NULL, 'e02c9988-2146-4c59-81e3-1b356f44b9c1', TRUE);

--

INSERT INTO time_served_abroad (id, city_id, person_id, boarding_date, start_date, end_date, law, law_date)
VALUES
    ('a98d2c21-1413-4c9f-860a-47e4a10ac32e', 'f7e232b3-700b-4376-8f2e-4aa9275a1016', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-12-10', '2023-12-22', NULL, 'D123456', '2023-11-01'),
    ('64a605e7-8a02-40d1-b2e1-6d2e82df28db', 'cdc50f66-0591-4c2f-a58e-8cb2aa3ce415', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2015-01-05', '2015-01-21', '2017-09-20', 'D654321', '2014-12-12'),
    ('bdd8eb13-05bf-44c5-b57f-b28d9ef155b8', 'f8efc2a0-62c3-49cd-bc7b-4088f9b59c68', '6628b9b5-cf0f-492d-834b-220c7aeb2b8c', '2017-02-15', '2017-02-27', '2020-05-01', 'D999999', '2017-01-22'),
    ('a6d61ff8-4f9e-4dc6-b761-d1d2e0ecad8b', 'f8efc2a0-62c3-49cd-bc7b-4088f9b59c68', '6628b9b5-cf0f-492d-834b-220c7aeb2b8c', '2022-01-01', '2022-01-17', '2024-02-17', 'D99977', '2021-12-22');

--

INSERT INTO fc_rf_by_roles (role_id, class_id, value, law, law_date)
VALUES
    ('48b81286-a5aa-493e-99a7-222d464ecf2e', 'c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', 150.00, 'PORTARIA MRE 402 / 2022', '2022-07-22'),
    ('48b81286-a5aa-493e-99a7-222d464ecf2e', '2c13d59c-fa5e-44a1-9abf-e92ac39c01b9', 100.00, 'PORTARIA MRE 402 / 2022', '2022-07-22'),
    ('48b81286-a5aa-493e-99a7-222d464ecf2e', '7c6cc408-570a-42b5-a693-12284c5e94b5', 90.00, 'PORTARIA MRE 402 / 2022', '2022-07-22'),
    ('b3f82163-fca4-45c5-8e1c-fc7b2f6e417d', 'c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', 80.00, 'PORTARIA MRE 402 / 2022', '2022-07-22'),
    ('b3f82163-fca4-45c5-8e1c-fc7b2f6e417d', '2c13d59c-fa5e-44a1-9abf-e92ac39c01b9', 70.00, 'PORTARIA MRE 402 / 2022', '2022-07-22'),
    ('b3f82163-fca4-45c5-8e1c-fc7b2f6e417d', '7c6cc408-570a-42b5-a693-12284c5e94b5', 70.00, 'PORTARIA MRE 402 / 2022', '2022-07-22');

--

INSERT INTO fc_rf_by_city (city_id, value, law, law_date)
VALUES
    ('cdc50f66-0591-4c2f-a58e-8cb2aa3ce415', 100.00, 'PORTARIA MRE 494 / 2023', '2023-09-20'),
    ('f8efc2a0-62c3-49cd-bc7b-4088f9b59c68', 49.00, 'PORTARIA MRE 494 / 2023', '2023-09-20'),
    ('f7e232b3-700b-4376-8f2e-4aa9275a1016', 68.00, 'PORTARIA MRE 494 / 2023', '2023-09-20');

--

INSERT INTO cf_limit_exchange_rate (id, law, law_date, value)
VALUES
    ('a8f10420-9b08-4c8b-a7d3-e74f34312e11', 'PORTARIA MRE 424 / 2022', '2022-12-09', 2.526),
    ('d9944d02-63da-4e6a-8bb2-d4d2c49c8f1f', 'PORTARIA MRE 369 / 2021', '2021-12-09', 2.362);

--

INSERT INTO cf_limit_value (id, law, law_date, value)
VALUES
    ('f4d44e4f-e2a7-4a2e-84f1-9816cf51f367', 'L14520/2023', '2023-01-09', 41650.92),
    ('a1e03c78-83f3-4e45-8cd4-72f2dd2d046c', 'L13752/2018', '2018-11-26', 39763.00);

--

INSERT INTO public.rf_payment_receipts (id, person_id, start_date, end_date, value, its_paid, rate)
VALUES
    ('6f7d87d7-1660-4ab6-a7f5-76a426b1e18a', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-09-10', '2023-09-09', 3400.0, TRUE, 1.0),
    ('e84c0c4a-d041-4d7e-aa88-7125a7b9c49c', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-09-11', '2023-10-10', 3400.0, TRUE, 1.0),
    ('3f99e7bb-9a4f-4dbb-88cd-3e71f415b1e9', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-10-11', '2023-11-10', 3400.0, TRUE, 1.0),
    ('f8de1e20-7b15-4ea7-8cc9-f05300abfa8f', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-11-11', '2023-12-10', 3400.0, FALSE, 1.0),
    ('2e9d95a9-0f1f-47c0-9a57-840a7122ec27', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-12-11', '2024-01-10', 3400.0, FALSE, 1.0),
    ('7b3a1a9e-6b3b-4b52-bc7d-f2f79be926b6', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2024-01-11', '2024-02-10', 3400.0, FALSE, 1.0);

--

UPDATE public.meta_payroll_items AS p
SET code = v.code
FROM (VALUES
    (CAST('0575e238-dc3f-49ce-a5ba-413418f030ec' AS uuid), '1056'),
    (CAST('a45e8206-e6e7-4996-8d41-49891af1f31e' AS uuid), '1057'),
    (CAST('12733c11-a07d-4675-bb54-7eec39152525' AS uuid), '1058'),
    (CAST('29afe6a6-1985-4711-b521-dbf1abcfcc6a' AS uuid), '1060'),
    (CAST('b3f3942d-2c0a-40f3-aa3e-93120fd49db7' AS uuid), '1523'),
    (CAST('89d36da2-d8da-4a3d-b3b9-a7e25ab4d422' AS uuid), '1524'),
    (CAST('a71f66ac-c2d4-43b6-9079-314391ab70f3' AS uuid), '1104'),
    (CAST('54cfcdf8-befe-4507-ba2b-c0618191b548' AS uuid), '1986'),
    (CAST('1bcdb645-91b3-4185-8ac7-edb9f645230a' AS uuid), '1985'),
    (CAST('8d89b4e8-8970-47b2-a914-c43b97bae49c' AS uuid), '3056'),
    (CAST('81e1e75e-a49c-418b-8dc8-e47bf9b2d65b' AS uuid), '1991'),
    (CAST('5edb4f6c-e8ec-4f40-8e45-7fc28c460abf' AS uuid), '1995'),
    (CAST('dc7dc82c-440d-43a6-b663-e127af2a6bce' AS uuid), '1990'),
    (CAST('4ff78775-18ec-4044-8349-d586804e0d0f' AS uuid), '3990'),
    (CAST('0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff' AS uuid), '1984'),
    (CAST('d89dec6c-4389-4221-b7d7-95912bf4e864' AS uuid), '3057')
) AS v(id, code)
WHERE p.id = v.id;










