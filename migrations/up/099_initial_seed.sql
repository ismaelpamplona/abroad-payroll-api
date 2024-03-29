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

INSERT INTO people (id, name, role_id, class_id, cpf, bank_id, bank_agency, bank_agency_account, has_retention_bonus, payroll_brl_pss)
VALUES
    ('a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 'Ana Silva', '48b81286-a5aa-493e-99a7-222d464ecf2e', 'c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', '01234567890', '12c8245d-1c63-4e03-8f2c-6c5979e6a3a4', '0123', '0123456-1', TRUE, 3642.55),
    ('6628b9b5-cf0f-492d-834b-220c7aeb2b8c', 'Carlos Santos', 'b3f82163-fca4-45c5-8e1c-fc7b2f6e417d', 'c1d1f5a2-3f05-4a49-8b47-5d2b0db10a44', '09876543210', '8e3639d1-0e04-4f98-9d6d-09a1b04f5369', '001', '9999-0', FALSE, 877.22),
    ('03a46f8a-028d-4b41-9d07-7c01612ecfd3', 'Camila Oliveira', '48b81286-a5aa-493e-99a7-222d464ecf2e', '2c13d59c-fa5e-44a1-9abf-e92ac39c01b9', '11111322223', '8e3639d1-0e04-4f98-9d6d-09a1b04f5369', '999', '11111-9', FALSE, 877.22);

--

INSERT INTO dependents (id, name, person_id, birth_date, start_date, end_date, type_id, ir)
VALUES
    ('2bf18dd3-370c-4f9b-8793-17e76c5f3d0a', 'Maria Oliveira', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2020-9-22', '2023-12-22', NULL, '72c5f0ac-3510-4a02-93cc-812f8b4991ce', FALSE),
    ('d51fbb2b-8815-4a90-82a2-34a963f9f8e7', 'Luiza Silva', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2017-10-1', '2023-12-22', NULL, '4a7aa6cf-8b19-4ee1-81ac-1bb335cb63e0', TRUE),
    ('f8f3aa4b-6629-4f43-804f-4d9a56248767', 'Lucas Santos', '6628b9b5-cf0f-492d-834b-220c7aeb2b8c', '2010-10-7', '2023-2-15', NULL, 'd5d7d1d7-31a3-41fc-b19f-2f6d150c61b9', TRUE),
    ('d94a87ef-2f3d-4b0e-9a2a-99b87c7a4b9c', 'Raoni Santos', '6628b9b5-cf0f-492d-834b-220c7aeb2b8c', '2010-10-7', '2023-2-15', NULL, 'e02c9988-2146-4c59-81e3-1b356f44b9c1', TRUE);

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
    ('48b81286-a5aa-493e-99a7-222d464ecf2e', '2c13d59c-fa5e-44a1-9abf-e92ac39c01b9', 100.00, 'PORTARIA MRE 402p/ 2022', '2022-07-22'),
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

INSERT INTO public.rf_payment_receipts (id, person_id, start_date, end_date, value, rate)
VALUES
    ('6f7d87d7-1660-4ab6-a7f5-76a426b1e18a', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-08-10', '2023-09-09', 3400.0, 1.0),
    ('e84c0c4a-d041-4d7e-aa88-7125a7b9c49c', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-09-11', '2023-10-10', 3400.0, 1.0),
    ('3f99e7bb-9a4f-4dbb-88cd-3e71f415b1e9', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-10-11', '2023-11-10', 3400.0, 1.0),
    ('f8de1e20-7b15-4ea7-8cc9-f05300abfa8f', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-11-11', '2023-12-10', 3600.0, 1.0),
    ('2e9d95a9-0f1f-47c0-9a57-840a7122ec27', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2023-12-11', '2024-01-10', 3800.0, 1.0),
    ('7b3a1a9e-6b3b-4b52-bc7d-f2f79be926b6', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '2024-01-01', '2024-01-31', 4200.0, 1.0);

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
    (CAST('d89dec6c-4389-4221-b7d7-95912bf4e864' AS uuid), '3057'),
    (CAST('2c98d9cd-9da3-412a-bf88-f8950aa67c1e' as uuid), '2403'),
    (CAST('9e2960e7-8b66-4040-94d0-0e123068d690' AS uuid), '2405'),
    (CAST('fb41cf1e-ea61-4f5b-95e7-43f818c388ec' AS uuid), '2430'),
    (CAST('00e59be5-c2cb-495c-92a2-bfc0adf718d9' AS uuid), '2482'),
    (CAST('dd9a0caa-18df-4d2a-9847-dfcfc82c228b' AS uuid), '2560'),
    (CAST('3c2fab19-14c4-49f9-912f-c6c67e0bc7a3' AS uuid), '2521'),
    (CAST('ab828ac1-c188-484c-9379-59c9f9d9b50f' AS uuid), '2522')
) AS v(id, code)
WHERE p.id = v.id;

-- 

INSERT INTO public.payroll_simulation (id, simulation_id, payroll_item, person_id, value, date)
VALUES
    ('3040d3ca-b2fd-4a28-bb68-f618b5a13cd8', '9a4339ad-c459-4bbc-a5db-72ff0237d28c', '0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 3400.0, '2023-09-09'),
    ('f6dc5763-3a9d-4a58-8379-81d97ec0d656', '9a4339ad-c459-4bbc-a5db-72ff0237d28c', '0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 3400.0, '2023-10-10'),
    ('c0c2def0-5a4c-4c92-a432-acba231e0729', '9a4339ad-c459-4bbc-a5db-72ff0237d28c', '0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 3400.0, '2023-11-10');

--

INSERT INTO public.payroll_closed (id, closed_id, payroll_item, person_id, value, date)
VALUES
    ('3040d3ca-b2fd-4a28-bb68-f618b5a13cd8', '917c1163-8c04-4263-9cd2-e405babd4cbb', '0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 3400.0, '2023-09-09'),
    ('f6dc5763-3a9d-4a58-8379-81d97ec0d656', '917c1163-8c04-4263-9cd2-e405babd4cbb', '0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 3400.0, '2023-10-10'),
    ('c0c2def0-5a4c-4c92-a432-acba231e0729', '917c1163-8c04-4263-9cd2-e405babd4cbb', '0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 3400.0, '2023-11-10');

--

INSERT INTO public.paid_rf_receipts(id, rf_receipt_id, payroll_closed_item_id)
VALUES
    ('53d69b91-26a5-456f-b131-82bed76b121f', '6f7d87d7-1660-4ab6-a7f5-76a426b1e18a', '3040d3ca-b2fd-4a28-bb68-f618b5a13cd8'),
    ('7a2fd36f-5342-44e8-be40-147bd2fdd094', 'e84c0c4a-d041-4d7e-aa88-7125a7b9c49c', 'f6dc5763-3a9d-4a58-8379-81d97ec0d656'),
    ('aec7cbed-cca9-429c-9afc-f31a9f6211fe', '3f99e7bb-9a4f-4dbb-88cd-3e71f415b1e9', 'c0c2def0-5a4c-4c92-a432-acba231e0729');

-- 

INSERT INTO public.manual_entries(id, person_id, payroll_item, value, start_date, end_date)
VALUES
    ('79d4e242-6d2a-4ef0-bae5-88e4f67f87c8', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', '5edb4f6c-e8ec-4f40-8e45-7fc28c460abf', 999.99, '2024-01-01', '2026-12-31');

--

INSERT INTO public.progressive_income_tax_table(id, from_value, to_value, tax_rate, parcel_deductible_value, law, law_date, start_from)
VALUES
    ('f639f793-c843-4f3b-8f34-e26d9a40736f', 0.0, 2259.20, 0.0, 0.0, 'MP 1.206/2024', '2024-2-6', '2024-2-1'),
    ('6f46f0bd-51ca-4ca4-ab99-3a2e555140a7', 2259.21, 2826.65, 0.075, 169.44, 'MP 1.206/2024', '2024-2-6', '2024-2-1'),
    ('bd0fc67d-ba6f-4b1e-bac5-63175ad7bab7', 2826.66, 3751.05, 0.15, 381.44, 'MP 1.206/2024', '2024-2-6', '2024-2-1'),
    ('cd8b9c40-7f0e-4408-9ae3-d50d72e1e455', 3751.05, 4664.68, 0.225, 662.77, 'MP 1.206/2024', '2024-2-6', '2024-2-1'),
    ('5afc05a3-304b-4b0e-b91a-914e6d7a985f', 4664.68, 'infinity'::float8, 0.275, 896.00, 'MP 1.206/2024', '2024-2-6', '2024-2-1'),
    ('ea6fd7b0-2c04-4a6d-84ab-037438905b5c', 0.0, 2112.0, 0.0, 0.0, 'MP 1.206/2024', '2024-2-6', '2023-5-1'),
    ('e97db80d-ca4b-45f2-9ee8-e97aea6c8365', 2112.1,  2826.65, 0.075, 158.40, 'MP 1.206/2024', '2024-2-6', '2023-5-1'),
    ('298a1fbe-6d4c-47a5-9ca0-617926e21f9f', 2826.66, 3751.05, 0.150, 370.40, 'MP 1.206/2024', '2024-2-6', '2023-5-1'),
    ('6fc88c3d-7d45-4ba8-927a-686e4fc8e578', 3751.06, 4664.68, 0.225, 651.73, 'MP 1.206/2024', '2024-2-6', '2023-5-1'),
    ('62cd86a4-40d2-44e4-a61c-2c4f92360650', 4664.68, 'infinity'::float8, 0.275, 884.96, 'MP 1.206/2024', '2024-2-6', '2023-5-1');

INSERT INTO public.people_open_fields(id, person_id, name, value)
VALUES
    ('0964bb3f-11e9-44af-a4f6-8fb8ba134fba', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 'Matrícula', '123456'),
    ('dd156b00-47e9-4429-8360-f0552b06f5b2', 'a188e92c-5a6e-4e36-81df-9b0714f4c7d8', 'Telefone', '61988990099');