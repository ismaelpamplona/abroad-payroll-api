CREATE TABLE fc_rf_by_roles (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    role UUID NOT NULL,
    class UUID NOT NULL,
    value NUMERIC(10, 4) NOT NULL,
    law VARCHAR(200) NOT NULL UNIQUE,
    law_date TIMESTAMP NOT NULL,
    FOREIGN KEY (role) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (class) REFERENCES classes(id) ON DELETE CASCADE,
    CONSTRAINT unique_role_class_combination UNIQUE (role, class)
);