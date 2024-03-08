-- public.people_open_fields definition

-- Drop table

-- DROP TABLE public.people_open_fields.;

CREATE TABLE public.people_open_fields (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    person_id uuid NULL,
    "name" varchar(100) NOT NULL,
    value  varchar(100) NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NULL,
    e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
    CONSTRAINT people_open_fields_pkey PRIMARY KEY (id),
    CONSTRAINT unique_name_person_id UNIQUE ("name", person_id)
);

-- Table Triggers

create trigger people_open_fields_updated_at before
update
    on
    public.people_open_fields for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger people_open_fields_update_etag before
insert
    or
update
    on
    public.people_open_fields for each row execute function update_etag();

-- public.people_open_fields foreign keys

ALTER TABLE public.people_open_fields ADD CONSTRAINT people_open_fields_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.people(id);
