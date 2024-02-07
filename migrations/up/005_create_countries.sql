-- public.countries definition

-- Drop table

-- DROP TABLE public.countries;

CREATE TABLE public.countries (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	"name" varchar(100) NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT countries_name_key UNIQUE (name),
	CONSTRAINT countries_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger countries_updated_at before
update
    on
    public.countries for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger countries_update_etag before
insert
    or
update
    on
    public.countries for each row execute function update_etag();

-- Permissions

ALTER TABLE public.countries OWNER TO postgres;
GRANT ALL ON TABLE public.countries TO postgres;