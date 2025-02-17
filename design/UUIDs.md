# Our UUIDs

New UUIDs are UUIDv4 (random UUIDs).
UUIDs from old Egiraffe are UUIDv8 (custom format).

## UUIDv8 specification for convertd IDs
Constant:
efefefef-000x-8ea2-0000-0000000xxxxx

The first x is the type of UUID (see below). 0000000xxxxx is replaced by the 0 padded byte representation of the ID from the old Egiraffe database.

Specific templates:
* University UUID: efefefef-0001-8ea2-0000-0000000xxxxx
* Course UUID: efefefef-0001-8ea2-0000-0000000xxxxx
* Prof UUID: (?): efefefef-0002-8ea2-0000-0000000xxxxx
* Upload UUID: efefefef-0003-8ea2-0000-0000000xxxxx
* File UUID: efefefef-0004-8ea2-0000-0000000xxxxx
* User UUID: efefefef-0005-8ea2-0000-0000000xxxxx
* EMail UUID: efefefef-0005-8ea2-0000-0000000xxxxx