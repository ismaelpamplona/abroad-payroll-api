CREATE TABLE banks (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    number VARCHAR(100) NOT NULL UNIQUE
);