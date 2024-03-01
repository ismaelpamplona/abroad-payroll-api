-- Step 1: Create the table
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

-- Step 2: Add foreign key constraints
ALTER TABLE public.payroll_closed 
ADD CONSTRAINT payroll_closed_payroll_item_fkey 
FOREIGN KEY (payroll_item) 
REFERENCES public.meta_payroll_items(id);

ALTER TABLE public.payroll_closed 
ADD CONSTRAINT payroll_closed_person_id_fkey 
FOREIGN KEY (person_id) 
REFERENCES public.people(id);

-- Assuming the execution context has the necessary privileges:

-- Step 3: Revoke general UPDATE and DELETE permissions
-- Adjust 'PUBLIC' as needed for your specific roles/environment
REVOKE DELETE, UPDATE ON public.payroll_closed FROM PUBLIC;

-- Grant DELETE and UPDATE privileges to the DBA role
-- Replace 'your_dba_role_here' with your actual DBA role name
GRANT DELETE, UPDATE ON public.payroll_closed TO your_dba_role_here;

-- Step 4: Create the trigger and function for date constraint
-- First, the function that will be called by the trigger
CREATE OR REPLACE FUNCTION check_date_constraint()
RETURNS TRIGGER AS $$
DECLARE
  latest_month DATE;
  check_passed BOOLEAN;
BEGIN
  -- Convert the session variable to boolean
  check_passed := (current_setting('custom.check_month_passed', true)::text = 'true');

  -- If the check has not passed, perform the check
  IF NOT check_passed THEN
    SELECT INTO latest_month DATE_TRUNC('month', MAX("date")) FROM public.payroll_closed;

    IF DATE_TRUNC('month', NEW."date") <= latest_month THEN
      RAISE EXCEPTION 'Cannot insert a record in the same month or earlier than the latest recorded month.';
    ELSE
      -- Mark that the check has passed for this transaction
      PERFORM set_config('custom.check_month_passed', 'true', false);
    END IF;
  END IF;

  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Then, the trigger that uses this function
CREATE TRIGGER check_date_before_insert
BEFORE INSERT ON public.payroll_closed
FOR EACH ROW EXECUTE FUNCTION check_date_constraint();
