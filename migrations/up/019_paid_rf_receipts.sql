-- public.paid_rf_receipts definition

-- Drop table

-- DROP TABLE public.paid_rf_receipts.;

CREATE TABLE public.paid_rf_receipts (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    rf_receipt_id uuid NOT NULL UNIQUE,
    payroll_closed_item_id uuid NOT NULL UNIQUE,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT paid_rf_receipts_pkey PRIMARY KEY (id)
);

-- public.paid_rf_receipts foreign keys

ALTER TABLE public.paid_rf_receipts ADD CONSTRAINT paid_rf_receipts_receipt_id_fkey FOREIGN KEY (rf_receipt_id) REFERENCES public.rf_payment_receipts(id);
ALTER TABLE public.paid_rf_receipts ADD CONSTRAINT payroll_closed_rf_receitps_payroll_id_fkey FOREIGN KEY (payroll_closed_item_id) REFERENCES public.payroll_closed(id);
