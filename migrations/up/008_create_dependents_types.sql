CREATE TABLE dependents_types (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(300) NOT NULL,
    value NUMERIC(5, 2) NOT NULL,
);