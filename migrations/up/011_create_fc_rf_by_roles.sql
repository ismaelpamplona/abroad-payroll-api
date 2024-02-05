-- public.fc_rf_by_roles definition

-- Drop table

-- DROP TABLE public.fc_rf_by_roles;

CREATE TABLE public.fc_rf_by_roles (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	role_id uuid NOT NULL,
	class_id uuid NOT NULL,
	value float8 NOT NULL,
	law varchar(200) NOT NULL,
	law_date date NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag uuid NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT fc_rf_by_roles_pkey PRIMARY KEY (id),
	CONSTRAINT unique_fc_rf_role_class UNIQUE (role_id, class_id)
);

-- Table Triggers

create trigger fc_rf_by_roles_updated_at before
update
    on
    public.fc_rf_by_roles for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger fc_rf_by_roles_update_etag before
insert
    or
update
    on
    public.fc_rf_by_roles for each row execute function update_etag();

-- Permissions

ALTER TABLE public.fc_rf_by_roles OWNER TO postgres;
GRANT ALL ON TABLE public.fc_rf_by_roles TO postgres;


-- public.fc_rf_by_roles foreign keys

ALTER TABLE public.fc_rf_by_roles ADD CONSTRAINT fc_rf_by_roles_class_fkey FOREIGN KEY (class_id) REFERENCES public.classes(id) ON DELETE RESTRICT;
ALTER TABLE public.fc_rf_by_roles ADD CONSTRAINT fc_rf_by_roles_role_fkey FOREIGN KEY (role_id) REFERENCES public.roles(id) ON DELETE RESTRICT;