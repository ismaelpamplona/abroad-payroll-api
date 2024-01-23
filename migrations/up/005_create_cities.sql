CREATE TABLE cities (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    country UUID NOT NULL,
    latitude NUMERIC(10, 6) NOT NULL,
    longitude NUMERIC(10, 6) NOT NULL,
    fc_rb NUMERIC(10, 4) NOT NULL,
    fc_irex NUMERIC(10, 4) NOT NULL,
    FOREIGN KEY (country) REFERENCES countries(id) ON DELETE CASCADE,
    CONSTRAINT unique_country_city UNIQUE (country, name)
);