CREATE TABLE fc_rf_by_city (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    city UUID NOT NULL UNIQUE,
    value NUMERIC(10, 4) NOT NULL,
    law VARCHAR(200) NOT NULL UNIQUE,
    law_date TIMESTAMP NOT NULL,
    FOREIGN KEY (city) REFERENCES cities(id) ON DELETE CASCADE
);