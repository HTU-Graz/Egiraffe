# Egiraffe Backend

To see what needs to get done, check out the work packages in [`work_pkgs.md`](../design/work_pkgs.md).

The backend of Egiraffe is written in [Rust](https://www.rust-lang.org/) using the [Axum](https://docs.rs/axum/latest/axum/) framework.

## Development

Use [rustup](https://rustup.rs/) to install Rust and Cargo, if you haven't already.

To install all dependencies, build, and run the backend in dev mode, run:

```zsh
# In the backend directory
cargo run
```

or to just build the server (dev mode), run:

```zsh
# In the backend directory
cargo build
```

For a Production build, add `--features prod` which will switch the code to production mode (different default settings; additional safety checks).

The first build will take a while, but subsequent builds will be much faster,
as the dependencies are cached, and incremental compilation is enabled.

Our SQL library `sqlx` utilizes optional macros (which we use) to check the SQL statements at compile time.  
It needs a working connection to a db to do so, so be sure to start docker-compose before. e.g.:

> ```zsh
> # For example, this code starts the db, launches VS Code(ium), and exits the shell
> sudo docker-compose up -d && code ~/Repos/egiraffe-ng && exit
> ```

<!-- TODO: Still needed? -->

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
