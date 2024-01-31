-- public.banks definition

-- Drop table

-- DROP TABLE public.banks;

CREATE TABLE public.banks (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	"name" varchar(100) NOT NULL,
	"number" varchar(100) NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT banks_name_key UNIQUE (name),
	CONSTRAINT banks_number_key UNIQUE (number),
	CONSTRAINT banks_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger banks_updated_at before
update
    on
    public.banks for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger banks_update_etag before
insert
    or
update
    on
    public.banks for each row execute function update_etag();

-- Permissions

ALTER TABLE public.banks OWNER TO postgres;
GRANT ALL ON TABLE public.banks TO postgres;