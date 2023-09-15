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

## License

[![GNU Affero General Public License v3.0](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)](https://www.gnu.org/licenses/agpl-3.0.html)

Egiraffe, "the software" (code, assets, design documents, and documentation) copyright (C) 2023 [HTU Graz](https://htugraz.at/)

Egiraffe is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Egiraffe is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with Egiraffe. If not, see <https://www.gnu.org/licenses/>, specifically <https://www.gnu.org/licenses/agpl-3.0.html>.
