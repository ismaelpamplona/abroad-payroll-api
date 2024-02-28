-- public.payroll_simulation definition

-- Drop table

-- DROP TABLE public.payroll_simulation;

CREATE TABLE public.payroll_simulation (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	simulation_id uuid NOT NULL,
	payroll_item uuid NOT NULL,
	person_id uuid NOT NULL,
	value float8 NOT NULL,
	"date" date NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT payroll_simulation_pkey PRIMARY KEY (id)
);

-- public.payroll_simulation foreign keys

ALTER TABLE public.payroll_simulation ADD CONSTRAINT payroll_simulation_payroll_item_fkey FOREIGN KEY (payroll_item) REFERENCES public.meta_payroll_items(id);
ALTER TABLE public.payroll_simulation ADD CONSTRAINT payroll_simulation_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.people(id);