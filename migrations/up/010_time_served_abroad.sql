-- public.time_served_abroad definition

-- Drop table

-- DROP TABLE public.time_served_abroad;

CREATE TABLE public.time_served_abroad (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	city uuid NOT NULL,
	person uuid NOT NULL,
	boarding_date timestamp NULL,
	start_date timestamp NULL,
	end_date timestamp NULL,
	law varchar(200) NOT NULL,
	law_date timestamp NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
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

ALTER TABLE public.time_served_abroad ADD CONSTRAINT time_served_abroad_city_fkey FOREIGN KEY (city) REFERENCES public.cities(id) ON DELETE CASCADE;
ALTER TABLE public.time_served_abroad ADD CONSTRAINT time_served_abroad_person_fkey FOREIGN KEY (person) REFERENCES public.people(id) ON DELETE CASCADE;