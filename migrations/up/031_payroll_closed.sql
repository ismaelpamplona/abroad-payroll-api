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
    e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4()
);

-- Create a trigger to enforce the unique constraint
CREATE OR REPLACE FUNCTION enforce_unique_payroll_month_year()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM public.payroll_closed
        WHERE payroll_item = NEW.payroll_item
        AND date_trunc('month', date) = date_trunc('month', NEW.date)
    ) THEN
        RAISE EXCEPTION 'Unique constraint violation for payroll_item and month-year';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Attach the trigger to the table
CREATE TRIGGER payroll_closed_unique_month_year
BEFORE INSERT ON public.payroll_closed
FOR EACH ROW
EXECUTE FUNCTION enforce_unique_payroll_month_year();

-- public.payroll_closed foreign keys

ALTER TABLE public.payroll_closed ADD CONSTRAINT payroll_closed_payroll_item_fkey FOREIGN KEY (payroll_item) REFERENCES public.meta_payroll_items(id);
