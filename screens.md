# Register process

## Meta payroll items

Register meta payroll items details

### Fields

- Code (e.g. 2804): `<input type="text">`
- Short name (e.g. PA): `<input type="text">`
- Description (e.g. Pensão Alimentícia): `<input type="text">`
- Transaction type (e.g. debit): `credit / debit`
- Consider for ir? (e.g. false): `true / false`

## Roles

Register roles.

### Fields

- Role name (e.g. Ministro): `<input type="text">`

# Classes

Register classes

### Fields

- Class name (e.g. Primeira Classe): `<input type="text">`

## Roles and Classes indexes

Register roles and classes indexes

### Fields

- Role (e.g. Ministro): `<select />`
- Class (e.g. Primeira Classe): `<select />`
- RB conversion factor (94.00): `<input type="number">`
- IREX conversion factor (80.00): `<input type="number">`

## Banks

Register banks

### Fields

- Name (e.g. Banco do Brasil): `<input type="text">`
- Number (e.g. 001): `<input type="text">`

## Countries

Register countries.

### Fields

- Country name (e.g. Alemanha): `<input type="text">`

## Cities

Register cities.

### Fields

- City name (e.g. Berlim): `<input type="text">`
- Country (e.g. Alemanha): `<select />`

## RF Conversion Factors by Roles

Register index of functional residency abroad by positions

### Fields

- Role (e.g. Ministro): `<select />`
- Class (e.g. Primeira classe): `<select />`
- Value (e.g. 150.00): `<input type="number">`
- Law (e.g. D123456): `<input type="text">`
- Law_date (e.g. 2022-01-01): `<input type="date">`

## RF Conversion Factors by Cities

Register index of functional residency abroad by cities

### Fields

- City (e.g. Berlim): `<select />`
- Value (e.g. 58.00): `<input type="number">`
- Law (e.g. D123456): `<input type="text">`
- Law_date (e.g. 2022-01-01): `<input type="date">`

## CF/88 Limit Exchange Rate

Register exchange rate for constitutional payment limit.

### Fields

- Law (e.g. PORTARIA MRE 424 / 2022): `<input type="text">`
- Law_date (e.g. 2022-12-09): `<input type="date">`
- Value (e.g. 2.526): `<input type="number">`

## CF/88 Limit Value

Register constitutional payment limit value.

### Fields

- Law (e.g. L14520/2023): `<input type="text">`
- Law_date (e.g. 2023-01-09): `<input type="date">`
- Value (e.g. 41650.92): `<input type="number">`

## Progressive Income Tax Table

Register progressive income tax table indexes.

### Fields

- From value: `<input type="number">`
- To_value: `<input type="number">`
- Tax_rate: `<input type="number">`
- Parcel_deductible_value: `<input type="number">`
- Law (e.g. D123456): `<input type="text">`
- Law_date (e.g. 2022-01-01): `<input type="date">`
- Start_from date: `<input type="date">`

## Dependents types

Register dependents types.

### Fields

- Type name (e.g. Esposa): `<input type="text">`
- Value (e.g. 0.1): `<input type="number">`

## People

Register people.

### Fields

- Person name (e.g. João Silva): `<input type="text">`
- Person role (e.g. Ministro): `<select />`
- Person class (e.g. Primeira classe): `<select />`
- Cpf (e.g. 6575966004sdsdasdasdas7): `<input type="text">`
- Person bank (e.g. Banco do Brasil): `<select />`
- Person bank agency (e.g. 1234): `<input type="text">`
- Person bank agency account (e.g. 123456): `<input type="text">`
- Has retention bonus? (e.g. True): `true / false`
- Person brl payroll pss (e.g. 4.500,00): `<input type="number">`
- Dependents:
  - Dependent name (e.g. Joana Silva Filho): `<input type="text">`
  - Dependent birth date (e.g. 2000-01-01): `<input type="text">`
  - Dependent start mission date (e.g. 2022-01-01): `<input type="date">`
  - Dependent end mission date (e.g. Null)`<input type="date">`
  - Dependent type (e.g. Esposa): `<select />`
  - Consider for ir? (e.g. True): `true / false`
- Time served abroad:
  - City (e.g. Berlim/Alemanha): `<select />`
  - Boarding_date (e.g. 2022-01-01): `<input type="date">`
  - Start_date (e.g. 2022-01-25): `<input type="date">`
  - End_date (e.g. Null): `<input type="date">`
  - Law (e.g. D123456): `<input type="text">`
  - Law_date (e.g. 2022-01-01): `<input type="date">`
- Receipts:
  - Start_date (e.g. 2022-01-01): `<input type="date">`
  - End_date (e.g. 2022-01-31): `<input type="date">`
  - Exchange rate (e.g. 4.92): `<input type="number">`
  - Recepit value (e.g. 6000.00): `<input type="number">`
- Vacation periods:
  - Start_date (e.g. 2024-01-01): `<input type="date">`,
  - End_date (e.g. 2024-01-31): `<input type="date">`,
  - Accrual_start_date (e.g. 2024-01-01): `<input type="date">`,
  - Accrual_end_date (e.g. 2024-12-31): `<input type="date">`,
  - Requested_salary_advance (e.g. True): `true / false`
  - Requested_christmas_bonus_advance (e.g. True): `true / false`
- Manual entries:
  - Payroll_item: `<select />`
  - value: `<input type="number">`
  - start_date: `<input type="date">`
  - end_date: `<input type="date">`
