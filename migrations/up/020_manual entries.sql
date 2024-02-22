-- public.manual_entries definition

-- Drop table

-- DROP TABLE public.manual_entries.;

CREATE TABLE public.manual_entries (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    person_id uuid NULL,
    payroll_item uuid NULL,
    value float8 NOT NULL,
    start_date date NOT NULL,
	end_date date NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT manual_entries_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger manual_entries_updated_at before
update
    on
    public.manual_entries for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger manual_entries_update_etag before
insert
    or
update
    on
    public.manual_entries for each row execute function update_etag();

-- public.manual_entries foreign keys

ALTER TABLE public.manual_entries ADD CONSTRAINT manual_entries_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.people(id);
ALTER TABLE public.manual_entries ADD CONSTRAINT manual_entries_payroll_item_fkey FOREIGN KEY (payroll_item) REFERENCES public.meta_payroll_items(id);
