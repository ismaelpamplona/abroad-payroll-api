-- public.classes definition

-- Drop table

-- DROP TABLE public.classes;

CREATE TABLE public.classes (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	"name" varchar(100) NOT NULL,
	updated_at timestamp NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT classes_name_key UNIQUE (name),
	CONSTRAINT classes_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger classes_update_etag before
insert
    or
update
    on
    public.classes for each row execute function update_etag();
create trigger classes_updated_at before
update
    on
    public.classes for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();

-- Permissions

ALTER TABLE public.classes OWNER TO postgres;
GRANT ALL ON TABLE public.classes TO postgres;