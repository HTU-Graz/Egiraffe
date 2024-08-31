# Egiraffe Backend

The backend of Egiraffe is written in [Rust](https://www.rust-lang.org/) using the [Axum](https://docs.rs/axum/latest/axum/) framework.

## Development

> [!IMPORTANT]
>
> **Notice: currently, you need to have a db running _WITH AN ENVIRONMENT VARIABLE WITH THE DB URL_ because the framework needs it**
>
> We're planing to alleviate this, but for now, it helps validate queries against the db schema in the IDE, and at compile time.
>
> ```zsh
> # For example, this code prepares the environment, launches VS Code(ium), and exits the shell
> export DATABASE_URL='postgresql://egiraffe:hunter2@localhost:5432/egiraffe?sslmode=disable' && code ~/Repos/egiraffe-ng && exit
> ```

To install all dependencies, build, and run the server, run:

```zsh
# In the backend directory
cargo run
```

or to just build the server, run:

```zsh
# In the backend directory
cargo build
```

The first build will take a while, but subsequent builds will be much faster,
as the dependencies are cached, and incremental compilation is enabled.

Use [rustup](https://rustup.rs/) to install Rust and Cargo, if you haven't already.

You may also need to define the database URL in the environment variable `DATABASE_URL`:

Our SQL library `sqlx` utilizes optional macros (which we use) to check the SQL statements at compile time.  
It needs a working connection to a db to do so.
Not providing a db url may confuse your LSP server; consider running:

```zsh
# For example
export DATABASE_URL='postgresql://egiraffe:hunter2@localhost:5432/egiraffe?sslmode=disable' # define the database URL
code # open VSCode
# then open the repo in VSCode
```

You may also need to apply the migrations before compilation, by running

```zsh
# in the backend directory
export DATABASE_URL='postgresql://egiraffe:hunter2@localhost:5432/egiraffe?sslmode=disable' # in case you haven't set that one yet
sqlx migrate run
```

## License

[![GNU Affero General Public License v3.0](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)](https://www.gnu.org/licenses/agpl-3.0.html)

Egiraffe, "the software" (code, assets, design documents, and documentation) copyright (C) 2023 [HTU Graz](https://htugraz.at/)

Egiraffe is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Egiraffe is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with Egiraffe. If not, see <https://www.gnu.org/licenses/>, specifically <https://www.gnu.org/licenses/agpl-3.0.html>.
