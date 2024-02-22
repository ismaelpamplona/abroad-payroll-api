-- public.vacation_periods definition

-- Drop table

-- DROP TABLE public.vacation_periods.;

CREATE TABLE public.vacation_periods (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    person_id uuid NULL,
    start_date date NOT NULL,
	end_date date NOT NULL,
    accrual_start_date date NOT NULL,
    accrual_end_date date NOT NULL,
    requested_salary_advance BOOLEAN NOT NULL DEFAULT false,
    requested_christmas_bonus_advance BOOLEAN NOT NULL DEFAULT false,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT vacation_periods_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger vacation_periods_updated_at before
update
    on
    public.vacation_periods for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger vacation_periods_update_etag before
insert
    or
update
    on
    public.vacation_periods for each row execute function update_etag();

-- public.vacation_periods foreign keys

ALTER TABLE public.vacation_periods ADD CONSTRAINT vacation_periods_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.people(id);
