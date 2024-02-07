-- public.dependents definition

-- Drop table

-- DROP TABLE public.dependents;

CREATE TABLE public.dependents (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	"name" varchar(300) NOT NULL,
	person_id uuid NOT NULL,
    birth_date date NOT NULL,
	start_date date NOT NULL,
	end_date date NULL,
	type_id uuid NOT NULL,
	ir bool NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT dependents_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger dependents_updated_at before
update
    on
    public.dependents for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger dependents_update_etag before
insert
    or
update
    on
    public.dependents for each row execute function update_etag();

-- Permissions

ALTER TABLE public.dependents OWNER TO postgres;
GRANT ALL ON TABLE public.dependents TO postgres;


-- public.dependents foreign keys

ALTER TABLE public.dependents ADD CONSTRAINT dependents_person_fkey FOREIGN KEY (person_id) REFERENCES public.people(id) ON DELETE RESTRICT;
ALTER TABLE public.dependents ADD CONSTRAINT dependents_type_fkey FOREIGN KEY (type_id) REFERENCES public.dependents_types(id) ON DELETE RESTRICT;


