# Egiraffe design documents

This directory contains design documents for Egiraffe, documenting technical decisions and the overall architecture of the project.

It contains a file named `egiraffe-erd.pgerd`, which is an [ERD](https://en.wikipedia.org/wiki/Entity%E2%80%93relationship_model) diagram of the database schema of Egiraffe, and was/is made and maintained using [pgAdmin](https://www.pgadmin.org/)'s integrated ERD editor.

For the time being, the old design documents are still available in the [`old`](old) directory.

## Directory structure

- [`database`](database): Database design documents
  - [`egiraffe-erd.pgerd`](database/egiraffe-erd.pgerd): ERD diagram of the database schema made with pgAdmin
  - [`egiraffe-schema-generated.sql`](database/egiraffe-schema-generated.sql): SQL script to generate the database schema
- [`api`](api): HTTP API design documents
  - [`egiraffe-api.yaml`](api/egiraffe-api.yaml): Insomnia API design document (see [Insomnia client](https://insomnia.rest/))
- [`old`](old): Old design documents, which are no longer maintained
