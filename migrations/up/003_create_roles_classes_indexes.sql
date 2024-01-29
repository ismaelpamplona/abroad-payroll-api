CREATE TABLE roles_classes_indexes (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    role UUID NOT NULL,
    class UUID NOT NULL,
    fc_rb DOUBLE PRECISION NOT NULL,
    fc_irex DOUBLE PRECISION NOT NULL,
    FOREIGN KEY (role) REFERENCES roles(id) ON DELETE CASCADE,
    FOREIGN KEY (class) REFERENCES classes(id) ON DELETE CASCADE,
    CONSTRAINT unique_role_class_combination UNIQUE (role, class)
);