# Egiraffe

Egiraffe is a website to download and share exam papers and other study materials.

Since 2005, when the first version of Egiraffe was released, it has been used by thousands of students in Austria.

Now, in 2023, Egiraffe is being rewritten from scratch to provide a more modern and easier to use experience.

## TL;DR

Assuming you installed all the tools (Node, Rust, pnpm, docker) just run:

```zsh
cd frontend
pnpm i
pnpm build

cd ../backend
sudo docker compose up -d
export DATABASE_URL='postgresql://egiraffe:hunter2@localhost:5432/egiraffe?sslmode=disable'
sqlx migrate run
cargo run
```

---

See the [requirements](./design/README.md#requirements) for a list of things needing to be done.

---

## Development

The project is structured in two parts: the frontend and the backend.

### Frontend

The frontend of Egiraffe is written in [Solid.js](https://www.solidjs.com/) and [Tailwind CSS](https://tailwindcss.com/).

See its code in the [`frontend`](frontend) directory, and its README in [`frontend/README.md`](frontend/README.md).

### Backend

The backend of Egiraffe is written in [Rust](https://www.rust-lang.org/) using the [Axum](https://docs.rs/axum/latest/axum/) framework.

See its code in the [`backend`](backend) directory, and its README in [`backend/README.md`](backend/README.md).

Our SQL library `sqlx` utilizes optional macros (which we use) to check the SQL statements at compile time.  
It needs a working connection to a db to do so.
Not providing a db url may confuse your LSP server; consider running:

> ```zsh
> # For example, this code prepares the environment, launches VS Code(ium), and exits the shell
> export DATABASE_URL='postgresql://egiraffe:hunter2@localhost:5432/egiraffe?sslmode=disable' && code ~/Repos/egiraffe-ng && exit
> ```

Update: these days, you can just tell rust-analyzer to restart, which seems to help.

## Devops

You can create a Docker image with both the frontend and the backend using the Dockerfile in the root directory of the project.

```zsh
# In the root directory of the project
docker build . -t egiraffe
```

then run it with

```zsh
docker run -it --rm --name egiraffe_dev egiraffe:latest
```

## License

[![GNU Affero General Public License v3.0](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)](https://www.gnu.org/licenses/agpl-3.0.html)

Egiraffe, "the software" (code, assets, design documents, and documentation) copyright (C) 2023 [HTU Graz](https://htugraz.at/)

Egiraffe is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Egiraffe is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with Egiraffe. If not, see <https://www.gnu.org/licenses/>, specifically <https://www.gnu.org/licenses/agpl-3.0.html>.
