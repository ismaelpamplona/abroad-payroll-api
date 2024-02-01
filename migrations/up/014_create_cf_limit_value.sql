-- public.cf_limit_value definition

-- Drop table

-- DROP TABLE public.cf_limit_value;

CREATE TABLE public.cf_limit_value (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	law varchar(200) NOT NULL,
	law_date timestamp NOT NULL,
	value float8 NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT cf_limit_value_law_key UNIQUE (law),
	CONSTRAINT cf_limit_value_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger cf_limit_value_updated_at before
update
    on
    public.cf_limit_value for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger cf_limit_value_update_etag before
insert
    or
update
    on
    public.cf_limit_value for each row execute function update_etag();

-- Permissions

ALTER TABLE public.cf_limit_value OWNER TO postgres;
GRANT ALL ON TABLE public.cf_limit_value TO postgres;