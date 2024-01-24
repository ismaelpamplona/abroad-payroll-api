CREATE TABLE payroll (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    is_retirement_bonus BOOLEAN NOT NULL,
    is_13_advance BOOLEAN NOT NULL,
    is_vacation BOOLEAN NOT NULL,
    is_rent BOOLEAN NOT NULL,
    exchange_rate NUMERIC(10, 4) NOT NULL
);
