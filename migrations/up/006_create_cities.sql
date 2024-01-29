CREATE TABLE cities (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    country UUID NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    fc_rb DOUBLE PRECISION NOT NULL,
    fc_irex DOUBLE PRECISION NOT NULL,
    FOREIGN KEY (country) REFERENCES countries(id) ON DELETE CASCADE,
    CONSTRAINT unique_country_city UNIQUE (country, name)
);