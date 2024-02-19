-- public.rf_payment_receipts definition

-- Drop table

-- DROP TABLE public.rf_payment_receipts;

CREATE TABLE public.rf_payment_receipts (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	person_id uuid NOT NULL,
	start_date date NOT NULL,
	end_date date NOT NULL,
	rate float8 NOT NULL,
	value float8 NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT rf_payment_receipts_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger rf_payment_receipts_updated_at before
update
    on
    public.rf_payment_receipts for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger rf_payment_receipts_update_etag before
insert
    or
update
    on
    public.rf_payment_receipts for each row execute function update_etag();


-- public.rf_payment_receipts foreign keys

ALTER TABLE public.rf_payment_receipts ADD CONSTRAINT rf_payment_receipts_person_fkey FOREIGN KEY (person_id) REFERENCES public.people(id) ON DELETE RESTRICT;

CREATE OR REPLACE FUNCTION check_date_range_overlap(
    new_start_date DATE,
    new_end_date DATE,
    person_id UUID,
    current_id UUID DEFAULT NULL
)
RETURNS BOOLEAN
LANGUAGE plpgsql
AS $$
DECLARE
    overlap_exists BOOLEAN;
BEGIN
    -- Initialize the overlap_exists variable as FALSE
    overlap_exists := FALSE;

    -- Check for overlapping date ranges excluding the current row (if specified)
    SELECT TRUE INTO overlap_exists
    FROM rf_payment_receipts
    WHERE
        person_id = check_date_range_overlap.person_id
        AND id <> check_date_range_overlap.current_id
        AND (
            (new_start_date BETWEEN start_date AND end_date)
            OR (new_end_date BETWEEN start_date AND end_date)
            OR (start_date BETWEEN new_start_date AND new_end_date)
        );

    -- Return TRUE if there is an overlap, FALSE otherwise
    RETURN NOT overlap_exists;
END;
$$;
