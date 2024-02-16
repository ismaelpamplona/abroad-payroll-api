-- public.dependents_types definition

-- Drop table

-- DROP TABLE public.dependents_types;

CREATE TABLE public.dependents_types (
	id uuid NOT NULL DEFAULT uuid_generate_v4(),
	"name" varchar(300) NOT NULL,
	value float8 NOT NULL,
	created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at timestamp NULL,
	e_tag varchar(100) NOT NULL DEFAULT uuid_generate_v4(),
	CONSTRAINT dependents_types_pkey PRIMARY KEY (id)
);

-- Table Triggers

create trigger dependents_types_updated_at before
update
    on
    public.dependents_types for each row
    when ((old.* is distinct
from
    new.*)) execute function update_updated_at();
create trigger dependents_types_update_etag before
insert
    or
update
    on
    public.dependents_types for each row execute function update_etag();

-- Permissions

ALTER TABLE public.dependents_types OWNER TO postgres;
GRANT ALL ON TABLE public.dependents_types TO postgres;


INSERT INTO dependents_types (id, name, value)
VALUES
('72c5f0ac-3510-4a02-93cc-812f8b4991ce', 'Esposa', 0.1),
('e02c9988-2146-4c59-81e3-1b356f44b9c1', 'Filho - inclusive enteados, adotivos, tutelados e curatelados - menor de 21 (vinte e um) anos ou que não receba remuneração ou inválido ou interdito', 0.05),
('d5d7d1d7-31a3-41fc-b19f-2f6d150c61b9', 'Filho - inclusive enteados, adotivos, tutelados e curatelados - estudante menor de 24 (vinte e quatro) anos que não receba remuneração ou inválido ou interdito', 0.05),
('4a7aa6cf-8b19-4ee1-81ac-1bb335cb63e0', 'Filha solteira - inclusive enteadas, adotivas, tuteladas e curateladas - que não receba remuneração', 0.05),
('c71d6a5e-3b05-4e53-987e-874daee65d94', 'Mãe viúva, que não receba remuneração;', 0.05),
('f6e646d3-99e3-4bc3-9d4e-72f2c75b3007', 'Mulher solteira, desquitada ou viúva, que viva, no mínimo há cinco anos, sob a dependência econômica do servidor solteiro, desquitado ou viúvo, e enquanto persistir o impedimento legal de qualquer das partes para se casar', 0.05);