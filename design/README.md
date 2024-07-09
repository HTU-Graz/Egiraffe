# Egiraffe design documents

This directory contains design documents for Egiraffe, documenting technical decisions and the overall architecture of the project.

This used to contain [ERD](https://en.wikipedia.org/wiki/Entity%E2%80%93relationship_model) diagrams of the database schema of Egiraffe.
However, as we now use hand-written SQL migration files, this has been dropped.

For the time being, the old design documents are still available in the [`old`](old) directory.

## Directory structure

- [`api`](api): HTTP API design documents (Bruno, see below)
- [`old`](old): Old design documents, which are no longer maintained
- `database`: This directory used to contain database design documents

## API design

The API design is documented in the [`api`](api) directory.

We use [Bruno](https://www.usebruno.com/) as our HTTP client, since it's open-source, doesn't do cloud stuff, and simply saves files locally in this Git repo.
Just open the `api` folder in Bruno to get started.

## License

[![GNU Affero General Public License v3.0](https://www.gnu.org/graphics/agplv3-with-text-162x68.png)](https://www.gnu.org/licenses/agpl-3.0.html)

Egiraffe, "the software" (code, assets, design documents, and documentation) copyright (C) 2023 [HTU Graz](https://htugraz.at/)

Egiraffe is free software: you can redistribute it and/or modify it under the terms of the [GNU Affero General Public License](/LICENSE.md) as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

Egiraffe is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the [GNU Affero General Public License](/LICENSE.md) for more details.

You should have received a copy of the [GNU Affero General Public License](/LICENSE.md) along with Egiraffe. If not, see <https://www.gnu.org/licenses/>, specifically <https://www.gnu.org/licenses/agpl-3.0.html>.

# Requirements

- Multiple universities
  - Be able to show an LV for more than 1 uni
- User accounts
  - Signup using uni-email
    - Prompt for WebAuthn creation by default on TPM devices
  - Login
    - Magic login link
    - Optional passwords
    - Fallback to magic login link via email
  - Pwd reset via email
  - Optional MFA
    - Allow users to enforce TOTP and/or WebAuthn and/or a password
  - Multiple emails
    - Every uni-mail grants access to that uni (multiple possible)
    - Attach personal emails (don't lose access to your account after your studies end)
  - Self-service (allow users to update their details by themselves)
- Permission system
  - Admins
  - Moderators
  - Regular users
  - Banned users
- Administration
  - Tested backups
  - Redundancy?
  - Creation of new unis
  - Promotion and revoking of moderator privileges
  - Access to user's data & credits
- Moderation
  - Scoped (uni or LV) access to moderation tools
  - Uni-wide mods should be able to create, update, merge, split, and retire LVs
  - Mods should be able to add new profs to the uni
  - Uni-wide mods need to review proposed new profs & LVs
  - Unscoped access for admins to all unis & LVs thereof
  - Ability for moderators to hide submissions from the public, or from the uploader and the public
- Uploads
  - Many social media platforms have posts, we call them "upload(s)"
  - Users can upload to any LV of all unis they are a part of
  - Uploads can/should have
    - Visibility via uploader: does the uploader agree for this to be public?
    - Visibility via the moderation team: is this ok to be public? (for review process)
    - Timestamp when the upload was made
    - Date of the exam, if any (optional)
    - The semester this applies to (infer from the date if possible, and display both if possible)
    - A type: exam, exam answers, homework, outline, etc.
    - A list of involved profs, if applicable (which prof held this LV this semester?)
    - A title to allow users to summarize which exam this is (e.g. 2/3 for VUs) or the modality
    - A description (textbox with line breaks)
    - Files attached to them
    - A price specified in ECs (capped by type, at least 0 ECs)
  - Every upload needs to have approval from the uploader _and_ from the mod team
- Files
  - Every file which gets uploaded is part of exactly one upload
  - As some uploads may need to be revised, we need to allow for multiple files
  - Only show the one, most recent file to the users who are not the uploader
  - Every file needs to have approval from the uploader _and_ them mod team
  - Users need to be able to propose new revisions to the mod team
  - Users and mods can each grant or revoke their approval of any file revision
  - Always show the newest file revision of any given upload with both approvals to users
  - Figure out a size limit
- Searchability
  - Indexing & search over all metadata described above
  - Full test search into files
    - Consider getting text out of PDFs
    - Consider performing OCR on images (zip files, or even images in PDFs)
- Credit system (ECs)
  - All users start out with 20 ECs
  - Every upload can be purchased by any user who is not the uploader of that upload
  - A purchase grants access to the upload, and at any given time, the visible file revision, if any
  - The metadata, but not any files can be viewed by non-purchasing users
  - The price paid for the purchase is recorded in the purchase
- User Interface
  - Mobile-first web interface
  - Not too cluttered/bloated
  - Suggestions for relevant items given user's interests (which LVs?)
  - Suggestions based on the general users per uni (tending LVs)
- Database
  - Define schema as list of migration files
    - Immutable SQL files
    - Automatically applied one after the other, if not already applied
    - Details in the library
  - Import all of the old data from Egiraffe-Legacy
    - Figure out the migration from the old to the new schema
    - Offer password resets
    - Optionally figure out how to import passwords
    - Otherwise, old users could get locked out
- Server infra
  - Package the app as Container & Nix package
  - For the future, consider scalability & LBs

# Implementation details

- Backend server
  - Serves APIs
  - Every request as HTTP(s) PUT
  - Separate routes for every action
  - Don't include parameters in the URL
  - Currently using Axum in Rust
  - Connecting to Database using sqlx in Rust
- Frontend
  - Single Page App
  - SolidJS
  - Consider Hydration & SSR for the future
  - Communicate using `fetch` & AJAX
- Database
  - Currently PostgreSQL
  - Hand-written schemas
    - sqlx migrates the db for us
    - Used to use ERD-Tools, we don't do that anymore
    - ERD-Tools are not stable enough & can't migrate in a clear way
- # Future considerations
  - Recommender system: Suggestions for relevant items given user's interests (which LVs?)
  - Server infra: For the future, consider scalability & LBs
  - Consider Hydration & SSR for the future
    - Consider migrating to Leptos to enable hydration
    - Would eliminate the SolidJS frontend
