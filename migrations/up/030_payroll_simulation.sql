-- CREATE TABLE payroll (
--     id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
--     is_retirement_bonus BOOLEAN NOT NULL,
--     is_13_advance BOOLEAN NOT NULL,
--     is_vacation BOOLEAN NOT NULL,
--     is_rent BOOLEAN NOT NULL,
--     exchange_rate float8 NOT NULL
-- );


id uuid NOT NULL DEFAULT uuid_generate_v4(),
payroll_item uuid (relationship with meta_payroll_items),
value float8 NOT NULL,
date (i need a date type)
created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
updated_at timestamp NULL,
e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
CONSTRAINT meta_payroll_items_pkey PRIMARY KEY (id),
CONSTRAINT unique_code UNIQUE (code),
CONSTRAINT unique_short_name UNIQUE (short_name)
