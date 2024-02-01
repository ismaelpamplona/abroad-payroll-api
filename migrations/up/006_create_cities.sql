-- public.cities definition

-- Drop table

-- DROP TABLE public.cities;

CREATE TABLE public.cities (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	"name" varchar(100) NOT NULL,
	country_id uuid NOT NULL,
	latitude float8 NOT NULL,
	longitude float8 NOT NULL,
	fc_rb float8 NOT NULL,
	fc_irex float8 NOT NULL,
    updated_at timestamp NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT cities_name_key UNIQUE (name),
	CONSTRAINT cities_pkey PRIMARY KEY (id),
	CONSTRAINT unique_country_city UNIQUE (country_id, name)
);

-- Table Triggers

create trigger cities_updated_at before
update
    on
    public.cities for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger cities_update_etag before
insert
    or
update
    on
    public.cities for each row execute function update_etag();

-- Permissions

ALTER TABLE public.cities OWNER TO postgres;
GRANT ALL ON TABLE public.cities TO postgres;


-- public.cities foreign keys

ALTER TABLE public.cities ADD CONSTRAINT cities_country_fkey FOREIGN KEY (country_id) REFERENCES public.countries(id) ON DELETE RESTRICT;
