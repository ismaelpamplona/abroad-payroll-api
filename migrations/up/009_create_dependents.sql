CREATE TABLE dependents (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    name VARCHAR(300) NOT NULL,
    person UUID NOT NULL,
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    type UUID NOT NULL,
    ir  BOOLEAN NOT NULL,
    FOREIGN KEY (person) REFERENCES people(id) ON DELETE CASCADE,
    FOREIGN KEY (type) REFERENCES dependents_types(id) ON DELETE CASCADE
);