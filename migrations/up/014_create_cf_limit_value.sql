CREATE TABLE cf_limit_value (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    law VARCHAR(200) NOT NULL UNIQUE,
    law_date TIMESTAMP NOT NULL,
    value NUMERIC(10, 4) NOT NULL
);