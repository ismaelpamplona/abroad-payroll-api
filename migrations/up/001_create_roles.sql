-- public.roles definition

-- Drop table

-- DROP TABLE public.roles;

CREATE TABLE public.roles (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	"name" varchar(100) NOT NULL,
	updated_at timestamp NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT roles_name_key UNIQUE (name),
	CONSTRAINT roles_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger roles_update_etag before
insert
    or
update
    on
    public.roles for each row execute function update_etag();
create trigger roles_updated_at before
update
    on
    public.roles for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();

-- Permissions

ALTER TABLE public.roles OWNER TO postgres;
GRANT ALL ON TABLE public.roles TO postgres;