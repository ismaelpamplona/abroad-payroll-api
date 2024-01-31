-- public.people definition

-- Drop table

-- DROP TABLE public.people;

CREATE TABLE public.people (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    "name" varchar(300) NOT NULL,
    "role" uuid NOT NULL,
    "class" uuid NOT NULL,
    cpf varchar(11) NOT NULL,
    bank uuid NOT NULL,
    bank_agency varchar(20) NOT NULL,
    bank_agency_account varchar(20) NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NULL,
    e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
    CONSTRAINT people_cpf_check CHECK (length(cpf) = 11),
    CONSTRAINT people_cpf_key UNIQUE (cpf),
    CONSTRAINT people_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger people_updated_at before
update
    on
    public.people for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger people_update_etag before
insert
    or
update
    on
    public.people for each row execute function update_etag();

-- Permissions

ALTER TABLE public.people OWNER TO postgres;
GRANT ALL ON TABLE public.people TO postgres;


-- public.people foreign keys

ALTER TABLE public.people ADD CONSTRAINT people_bank_fkey FOREIGN KEY (bank) REFERENCES public.banks(id) ON DELETE CASCADE;
ALTER TABLE public.people ADD CONSTRAINT people_class_fkey FOREIGN KEY ("class") REFERENCES public.classes(id) ON DELETE CASCADE;
ALTER TABLE public.people ADD CONSTRAINT people_role_fkey FOREIGN KEY ("role") REFERENCES public.roles(id) ON DELETE CASCADE;






