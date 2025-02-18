# UUIDs in Egiraffe

The old version of Egiraffe (1.x written in PHP, aka Egiraffe Legacy) used integer IDs instead of UUIDs to identify entities in its database.

Since we have imported data from EG Legacy, we use UUID v8 to create a bijective mapping between UUIDs and the old IDs.
This is achieved with the following bit pattern:

## Mapping

All new data in Egiraffe is identified using UUID v4.
The imported data has a bijective mapping from integers to UUID v8.

UUIDs consist of 128 bits.
We record the table of origin from the imported data.

The UUID v8 are structured as follows:

efefefef-000x-8ea2-0000-0000000xxxxx

We're using

```
xxxxxxxx-xxxx-8xxx-8xxx-xxxxxxxxxxxx
```

as our fixed value, where the x-es can be assigned to data.

Technically speaking, we're wasing 2 bit, as the second 8 could also be a 9, A, or B.

We now put the table number into the byte below marked as TT,
and the up-to 32bit legacy ID into the last 4 bytes.

```
xxxxxxxx-xxTT-8xxx-8xxx-xxxxIIIIIIII
```

The rest is filled with zeros, resulting in

```
00000000-00TT-8000-8000-0000IIIIIIII
```

TL;DR: the table number is in octet 5, and the legacy ID is in octets 12-16 (inc, excl, big endian)

## Legacy Table Numbers

The tables of the Legacy DB are numbered as follows:

| Table name | Mapping number |
| ---------- | -------------- |
| University | 0              |
| Course     | 1              |
| Prof       | 2              |
| Upload     | 3              |
| File       | 4              |
| User       | 5              |
| Email      | 6              |
