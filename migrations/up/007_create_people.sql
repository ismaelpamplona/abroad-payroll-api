CREATE TABLE people (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(300) NOT NULL,
    role UUID NOT NULL,
    class UUID NOT NULL,
    cpf VARCHAR(11) UNIQUE CHECK (LENGTH(cpf) = 11) NOT NULL,
    bank UUID NOT NULL,
    bank_agency VARCHAR(20) NOT NULL,
    bank_agency_account VARCHAR(20) NOT NULL,
    FOREIGN KEY (role) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (class) REFERENCES classes(id) ON DELETE CASCADE,
    FOREIGN KEY (bank) REFERENCES banks(id) ON DELETE CASCADE
);