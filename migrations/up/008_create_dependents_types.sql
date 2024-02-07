-- public.dependents_types definition

-- Drop table

-- DROP TABLE public.dependents_types;

CREATE TABLE public.dependents_types (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	"name" varchar(300) NOT NULL,
	value float8 NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT dependents_types_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger dependents_types_updated_at before
update
    on
    public.dependents_types for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger dependents_types_update_etag before
insert
    or
update
    on
    public.dependents_types for each row execute function update_etag();

-- Permissions

ALTER TABLE public.dependents_types OWNER TO postgres;
GRANT ALL ON TABLE public.dependents_types TO postgres;