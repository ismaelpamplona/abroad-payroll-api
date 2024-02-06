-- public.payroll_closed definition

-- Drop table

-- DROP TABLE public.payroll_closed;

CREATE TABLE public.payroll_closed (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	payroll_item uuid NULL,
	value float8 NOT NULL,
	"date" date NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag uuid NOT NULL DEFAULT uuid_generate_v4()
);

-- Table Triggers

create trigger payroll_closed_unique_month_year before
insert
    on
    public.payroll_closed for each row execute function enforce_unique_payroll_month_year();


-- public.payroll_closed foreign keys

ALTER TABLE public.payroll_closed ADD CONSTRAINT payroll_closed_payroll_item_fkey FOREIGN KEY (payroll_item) REFERENCES public.meta_payroll_items(id);