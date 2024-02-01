-- public.fc_rf_by_city definition

-- Drop table

-- DROP TABLE public.fc_rf_by_city;

CREATE TABLE public.fc_rf_by_city (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	city_id uuid NOT NULL,
	value float8 NOT NULL,
	law varchar(200) NOT NULL,
	law_date timestamp NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT fc_rf_by_city_city_key UNIQUE (city_id),
	CONSTRAINT fc_rf_by_city_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger fc_rf_by_city_updated_at before
update
    on
    public.fc_rf_by_city for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger fc_rf_by_city_update_etag before
insert
    or
update
    on
    public.fc_rf_by_city for each row execute function update_etag();

-- Permissions

ALTER TABLE public.fc_rf_by_city OWNER TO postgres;
GRANT ALL ON TABLE public.fc_rf_by_city TO postgres;


-- public.fc_rf_by_city foreign keys

ALTER TABLE public.fc_rf_by_city ADD CONSTRAINT fc_rf_by_city_city_fkey FOREIGN KEY (city_id) REFERENCES public.cities(id) ON DELETE RESTRICT;