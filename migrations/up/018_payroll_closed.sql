-- public.payroll_closed definition

-- Drop table

-- DROP TABLE public.payroll_closed;

CREATE TABLE public.payroll_closed (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	closed_id uuid NOT NULL,
	payroll_item uuid NOT NULL,
	person_id uuid NOT NULL,
	value float8 NOT NULL,
	"date" date NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT payroll_closed_pkey PRIMARY KEY (id)
);

-- public.payroll_closed foreign keys

ALTER TABLE public.payroll_closed ADD CONSTRAINT payroll_closed_payroll_item_fkey FOREIGN KEY (payroll_item) REFERENCES public.meta_payroll_items(id);
ALTER TABLE public.payroll_closed ADD CONSTRAINT payroll_closed_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.people(id);