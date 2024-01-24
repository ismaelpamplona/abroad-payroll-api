# develop_data_registration_crud_endpoints (Issue)

- [ ] Cargos (`roles`)

  - Name

- [ ] Classes

  - Name

- [ ] Bancos (`banks`)

  - Nome
  - Número

- [ ] Países (`countries`)

  - Nome

- [ ] Postos (cidades) (`cities`)

  - País (uuid)
  - Localização (lat long)
  - Fator de conversão da retribuição básica (5809)
  - Fator de conversão da IREX (71733)

- [ ] Personal data (`people`)

  - Nome
  - Role (uuid)
  - Class (uuid)
  - CPF
  - Dados bancários
    - Banco (uuid)
    - Agência
    - Conta
  - ATS
  - Dependentes
  - Dependentes IR

- [ ] person_cities (`person_cities`)

  - Person (uuid)
  - City (uuid)
  - Data de embarque
  - Data de partida

- [ ] people_time_served_abroad (`people_missions_abroad`)

  - Person (uuid)
  - City (uuid)
  - Data de embarque
  - Data de partida

- [ ] Cargos (`roles_indexes`)

  - Cargo (role) (uuid)
  - Classe (class) (uuid)
  - Fator de conversão da retribuição básica (5809)
  - Fator de conversão da IREX (71733)

- [ ] Fator de conversão da residência funcional por cargos (GAP - Portaria MRE 494 22/07/22)

  - role (uuid)
  - class (uuid)
  - value
  - law_number
  - law_date

- [ ] Fator de conversão da residência funcional por posto (GAP - Portaria MRE 494 22/07/22)

  - País (uuid)
  - cidade (uuid)
  - value
  - law_number
  - law_date

- [ ] Taxa de cambio teto constitucional (art. 37, XI, CF | Portaria 693 22/12/2015)

  - law
  - law_date
  - value

- [ ] Teto constitucional (Art. 37, XL, CF)

- [ ] Tabela imposto de renda

- [ ] Payroll data

  - Abono (bool)
  - Antecipação de gratificação natalina (bool)
  - Férias (bool)
  - Cotação

- [ ] Pensão alimentícia

- [ ] Consignações
