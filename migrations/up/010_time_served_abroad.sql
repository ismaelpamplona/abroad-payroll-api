CREATE TABLE dependents (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    city UUID NOT NULL,
    person UUID NOT NULL,
    boarding_date TIMESTAMP,
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    law VARCHAR(200) NOT NULL UNIQUE,
    law_date TIMESTAMP NOT NULL,
    FOREIGN KEY (city) REFERENCES cities(id) ON DELETE CASCADE,
    FOREIGN KEY (person) REFERENCES people(id) ON DELETE CASCADE
);