-- public.people definition

-- Drop table

-- DROP TABLE public.people;

CREATE TABLE public.people (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    name varchar(300) NOT NULL,
    role_id uuid NOT NULL,
    class_id uuid NOT NULL,
    cpf varchar(11) NOT NULL,
    bank_id uuid NOT NULL,
    bank_agency varchar(20) NOT NULL,
    bank_agency_account varchar(20) NOT NULL,
    has_retention_bonus BOOLEAN NOT NULL DEFAULT false,
    payroll_brl_pss float8 NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NULL,
    e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
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

ALTER TABLE public.people ADD CONSTRAINT people_bank_fkey FOREIGN KEY (bank_id) REFERENCES public.banks(id) ON DELETE RESTRICT;
ALTER TABLE public.people ADD CONSTRAINT people_class_fkey FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE RESTRICT;
ALTER TABLE public.people ADD CONSTRAINT people_role_fkey FOREIGN KEY (role_id) REFERENCES public.roles(id) ON DELETE RESTRICT;






