-- public.progressive_income_tax_table definition

-- Drop table

-- DROP TABLE public.progressive_income_tax_table.;

CREATE TABLE public.progressive_income_tax_table (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    from_value float8 NOT NULL,
	to_value float8 NOT NULL,
    tax_rate float8 NOT NULL,
    parcel_deductible_value float8 NOT NULL,
    law varchar(200) NOT NULL,
	law_date date NOT NULL,
    start_from date NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT progressive_income_tax_table_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger progressive_income_tax_table_updated_at before
update
    on
    public.progressive_income_tax_table for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger progressive_income_tax_table_update_etag before
insert
    or
update
    on
    public.progressive_income_tax_table for each row execute function update_etag();