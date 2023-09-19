# Egiraffe design documents

This directory contains design documents for Egiraffe, documenting technical decisions and the overall architecture of the project.

It contains a file named `egiraffe-erd.pgerd`, which is an [ERD](https://en.wikipedia.org/wiki/Entity%E2%80%93relationship_model) diagram of the database schema of Egiraffe, and was/is made and maintained using [pgAdmin](https://www.pgadmin.org/)'s integrated ERD editor.

For the time being, the old design documents are still available in the [`old`](old) directory.

## Directory structure

- [`database`](database): Database design documents
  - [`egiraffe-erd.pgerd`](database/egiraffe-erd.pgerd): ERD diagram of the database schema made with pgAdmin
  - [`egiraffe-schema-generated.sql`](database/egiraffe-schema-generated.sql): SQL script to generate the database schema
- [`api`](api): HTTP API design documents
- [`old`](old): Old design documents, which are no longer maintained

## API design

The API design is documented in the [`api`](api) directory.

Since [Insomnia](https://insomnia.rest/) has the rather bizarre habit of saving files in a ✨ _mystery_ ✨ format/location,
and doesn't have a built-in way to meaningfully save & load files to a specific location,
an Insomnia plugin, called [_Git integration_](https://github.com/Its-treason/insomnia-plugin-git-integration), is used to save
and load the API design documents, requests, and other data to/from the [`api`](api) directory.

At least that's the plan for when it's finished.
For now, Insomnia's built-in import/export functionality is used to save the API design documents to the [`api`](api) directory
as `api/egiraffe-api.yaml`.

<!--
### Working with the plugin

1. Open [Insomnia](https://insomnia.rest/)
2. Go to `Application` > `Preferences` > `Plugins`
3. Copy and paste `insomnia-plugin-free-sync` into the `npm package name` field
4. Click `Install Plugin`
5. Close the `Preferences` window
6. Open or create a collection called `Egiraffe v1 API`
   <!-- (its contents will be replaced/overwritten by the plugin) - ->
   (the import will create a duplicate collection, the old one can be deleted)
7. In the middle of the top bar, click the _Egiraffe v1 API_ dropdown
8. Click _Free sync: Configuration_
9. Paste `$REPO_ROOT/design/api`, where `$REPO_ROOT` is the root directory of the repository, into the path field
10. Close the dialog
11. Open the dropdown from step 7 again
12. Click
-->

## License

[![GNU Affero General Public License v3.0](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)](https://www.gnu.org/licenses/agpl-3.0.html)

Egiraffe, "the software" (code, assets, design documents, and documentation) copyright (C) 2023 [HTU Graz](https://htugraz.at/)

Egiraffe is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Egiraffe is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with Egiraffe. If not, see <https://www.gnu.org/licenses/>, specifically <https://www.gnu.org/licenses/agpl-3.0.html>.
