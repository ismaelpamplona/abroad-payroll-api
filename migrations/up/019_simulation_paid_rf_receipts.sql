-- public.simulation_paid_rf_receipts definition

-- Drop table

-- DROP TABLE public.simulation_paid_rf_receipts.;

CREATE TABLE public.simulation_paid_rf_receipts (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    rf_receipt_id uuid NOT NULL UNIQUE,
    payroll_simulation_item_id uuid NOT NULL UNIQUE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT simulation_paid_rf_receipts_pkey PRIMARY KEY (id)
);

-- public.simulation_paid_rf_receipts foreign keys

ALTER TABLE public.simulation_paid_rf_receipts ADD CONSTRAINT simulation_paid_rf_receipts_receipt_id_fkey FOREIGN KEY (rf_receipt_id) REFERENCES public.rf_payment_receipts(id);
ALTER TABLE public.simulation_paid_rf_receipts ADD CONSTRAINT payroll_simulation_rf_receitps_payroll_id_fkey FOREIGN KEY (payroll_simulation_item_id) REFERENCES public.payroll_simulation(id);





