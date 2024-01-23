CREATE TABLE roles_indexes (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    role UUID NOT NULL,
    class UUID NOT NULL,
    fc_rb NUMERIC(10, 4) NOT NULL,
    fc_irex NUMERIC(10, 4) NOT NULL,
    FOREIGN KEY (role) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (class) REFERENCES classes(id) ON DELETE CASCADE,
    CONSTRAINT unique_role_class_combination UNIQUE (role, class)
);