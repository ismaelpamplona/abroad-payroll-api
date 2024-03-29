-- public.time_served_abroad definition

-- Drop table

-- DROP TABLE public.time_served_abroad;

CREATE TABLE public.time_served_abroad (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	city_id uuid NOT NULL,
	person_id uuid NOT NULL,
	boarding_date date NULL,
	start_date date NOT NULL,
	end_date date NULL,
	law varchar(200) NOT NULL,
	law_date date NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT time_served_abroad_law_key UNIQUE (law),
	CONSTRAINT time_served_abroad_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger time_served_abroad_updated_at before
update
    on
    public.time_served_abroad for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger time_served_abroad_update_etag before
insert
    or
update
    on
    public.time_served_abroad for each row execute function update_etag();

-- Permissions

ALTER TABLE public.time_served_abroad OWNER TO postgres;
GRANT ALL ON TABLE public.time_served_abroad TO postgres;


-- public.time_served_abroad foreign keys

ALTER TABLE public.time_served_abroad ADD CONSTRAINT time_served_abroad_city_fkey FOREIGN KEY (city_id) REFERENCES public.cities(id) ON DELETE RESTRICT;
ALTER TABLE public.time_served_abroad ADD CONSTRAINT time_served_abroad_person_fkey FOREIGN KEY (person_id) REFERENCES public.people(id) ON DELETE RESTRICT;