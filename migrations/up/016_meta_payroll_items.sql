-- public.meta_payroll_items definition

-- Drop table

-- DROP TABLE public.meta_payroll_items;
CREATE TYPE transaction_type AS ENUM ('credit', 'debit');

CREATE TABLE public.meta_payroll_items (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	code varchar(30) NULL,
	short_name varchar(10) NOT NULL,
	description varchar(100) NOT NULL,
	"transaction_type" public."transaction_type" NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT meta_payroll_items_pkey PRIMARY KEY (id),
	CONSTRAINT unique_code UNIQUE (code),
	CONSTRAINT unique_short_name UNIQUE (short_name)
);

-- Table Triggers

create trigger meta_payroll_items_updated_at before
update
    on
    public.meta_payroll_items for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger meta_payroll_items_update_etag before
insert
    or
update
    on
    public.meta_payroll_items for each row execute function update_etag();


INSERT INTO public.meta_payroll_items (id, code, short_name, description, transaction_type) 
VALUES 
('0575e238-dc3f-49ce-a5ba-413418f030ec', NULL, 'RB', 'Retribuição básica', 'credit'),
('a45e8206-e6e7-4996-8d41-49891af1f31e', NULL, 'IREX', 'Indenização de representação no exterior', 'credit'),
('12733c11-a07d-4675-bb54-7eec39152525', NULL, 'GETS', 'Gratificação no exterior por tempo de serviço', 'credit'),
('29afe6a6-1985-4711-b521-dbf1abcfcc6a', NULL, 'AF', 'Auxílio familiar', 'credit'),
('b3f3942d-2c0a-40f3-aa3e-93120fd49db7', NULL, 'AP', 'Abono permanência', 'credit'),
('89d36da2-d8da-4a3d-b3b9-a7e25ab4d422', NULL, 'APGN', 'Abono permanência gratificação natalina', 'credit'),
('a71f66ac-c2d4-43b6-9079-314391ab70f3', NULL, 'AGN', 'Antecipação gratificação natalina', 'credit'),
('54cfcdf8-befe-4507-ba2b-c0618191b548', NULL, 'GN', 'Gratificação natalina', 'credit'),
('1bcdb645-91b3-4185-8ac7-edb9f645230a', NULL, 'AFE', 'Adicional de férias', 'credit'),
('8d89b4e8-8970-47b2-a914-c43b97bae49c', NULL, 'DVE', 'Diferença vencimento no exterior', 'credit'),
('81e1e75e-a49c-418b-8dc8-e47bf9b2d65b', NULL, 'GNP', 'Gratificação natalina proporcional', 'credit'),
('5edb4f6c-e8ec-4f40-8e45-7fc28c460abf', NULL, 'DEAGN', 'Despesas com exercício anterior da gratificação natalina', 'credit'),
('dc7dc82c-440d-43a6-b663-e127af2a6bce', NULL, 'DEAT', 'Despesas com exercício anterior (tributável)', 'credit'),
('4ff78775-18ec-4044-8349-d586804e0d0f', NULL, 'DEANT', 'Despesas com exercício anterior (não tributável)', 'credit'),
('0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff', NULL, 'IRFE', 'Indenização com residencia funcional', 'credit'),
('d89dec6c-4389-4221-b7d7-95912bf4e864', NULL, 'DIREX', 'Diferença IREX', 'credit'),
('2c98d9cd-9da3-412a-bf88-f8950aa67c1e', NULL, 'PSS', 'Contribuição para o Plano de Seguridade Social', 'debit');

CREATE OR REPLACE FUNCTION prevent_deletion()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.id = ANY (ARRAY['0575e238-dc3f-49ce-a5ba-413418f030ec', 'a45e8206-e6e7-4996-8d41-49891af1f31e', '12733c11-a07d-4675-bb54-7eec39152525', '29afe6a6-1985-4711-b521-dbf1abcfcc6a', 'b3f3942d-2c0a-40f3-aa3e-93120fd49db7', '89d36da2-d8da-4a3d-b3b9-a7e25ab4d422', 'a71f66ac-c2d4-43b6-9079-314391ab70f3', '54cfcdf8-befe-4507-ba2b-c0618191b548', '1bcdb645-91b3-4185-8ac7-edb9f645230a', '8d89b4e8-8970-47b2-a914-c43b97bae49c', '81e1e75e-a49c-418b-8dc8-e47bf9b2d65b', '5edb4f6c-e8ec-4f40-8e45-7fc28c460abf', 'dc7dc82c-440d-43a6-b663-e127af2a6bce', '4ff78775-18ec-4044-8349-d586804e0d0f', '0750f2eb-85ec-4bc5-ab7d-bf5bdcc5beff', 'd89dec6c-4389-4221-b7d7-95912bf4e864']::uuid[]) THEN
        RAISE EXCEPTION 'Deletion of this row is not allowed.';
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER no_delete_trigger
BEFORE DELETE ON public.meta_payroll_items
FOR EACH ROW EXECUTE FUNCTION prevent_deletion();
