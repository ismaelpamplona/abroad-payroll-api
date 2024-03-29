-- public.roles_classes_indexes definition

-- Drop table

-- DROP TABLE public.roles_classes_indexes;

CREATE TABLE public.roles_classes_indexes (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	role_id uuid NOT NULL,
	class_id uuid NOT NULL,
	fc_rb float8 NOT NULL,
	fc_irex float8 NOT NULL,
	updated_at timestamp NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT roles_classes_indexes_pkey PRIMARY KEY (id),
	CONSTRAINT unique_role_class_combination UNIQUE (role_id, class_id)
);

-- Table Triggers

create trigger roles_classes_indexes_updated_at before
update
    on
    public.roles_classes_indexes for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger roles_classes_indexes_update_etag before
insert
    or
update
    on
    public.roles_classes_indexes for each row execute function update_etag();

-- Permissions

ALTER TABLE public.roles_classes_indexes OWNER TO postgres;


-- public.roles_classes_indexes foreign keys

ALTER TABLE public.roles_classes_indexes ADD CONSTRAINT roles_classes_indexes_class_fkey FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE RESTRICT;
ALTER TABLE public.roles_classes_indexes ADD CONSTRAINT roles_classes_indexes_role_fkey FOREIGN KEY (role_id) REFERENCES public.roles(id) ON DELETE RESTRICT;