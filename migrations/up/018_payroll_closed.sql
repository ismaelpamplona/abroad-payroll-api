-- public.payroll_closed definition

-- Drop table

-- DROP TABLE public.payroll_closed;

CREATE TABLE public.payroll_closed (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    closed_id uuid NOT NULL,
    payroll_item uuid NULL,
    person_id uuid NULL,
    value float8 NOT NULL,
    "date" date NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT payroll_closed_pkey PRIMARY KEY (id)
);

-- Create a trigger to enforce the unique constraint
CREATE OR REPLACE FUNCTION enforce_unique_payroll_person_month_year()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM public.payroll_closed
        WHERE person_id = NEW.person_id
          AND payroll_item = NEW.payroll_item
          AND date_trunc('month', date) = date_trunc('month', NEW.date)
    ) THEN
        RAISE EXCEPTION 'Unique constraint violation for person_id, payroll_item, and month-year combination.';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER payroll_closed_unique_person_payroll_item_month_year
BEFORE INSERT OR UPDATE ON public.payroll_closed
FOR EACH ROW
EXECUTE FUNCTION enforce_unique_payroll_person_month_year();


-- public.payroll_closed foreign keys

ALTER TABLE public.payroll_closed ADD CONSTRAINT payroll_closed_payroll_item_fkey FOREIGN KEY (payroll_item) REFERENCES public.meta_payroll_items(id);
ALTER TABLE public.payroll_closed ADD CONSTRAINT payroll_closed_person_id_fkey FOREIGN KEY (person_id) REFERENCES public.people(id);
