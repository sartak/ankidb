# Changelog

## NEXT

- add `where_tag` query method

## [0.5.0] - 2025-03-02

- _breaking_: upgrade to 2024 edition, which bumps MSRV to 1.85

- add `parse_fields` and `parse_tags` functions
- improve types for `notetypes` doctest

## [0.4.1] - 2024-07-25

- add `where_nid` and `where_cid` query methods

## [0.4.0] - 2024-07-22

- _breaking_: update `sea-query`, `postgres-types`, and `serde` dependencies

- add `notetypes` method

## [0.3.0] - 2023-12-16

- _breaking_: update `sea-query`, `postgres-types`, and `serde` dependencies

- add `where_fields_like` and `where_fields_match` query methods

## [0.2.1] - 2023-11-20

- implement `sea_query::Nullable` for id types
- add optional `serde` feature
- add optional `postgres-types` feature

## [0.2.0] - 2023-09-16

- _breaking_: add `sea-query` and `sea-query-rusqlite` dependencies
- _breaking_: stop re-exporting id structs and instead expose `ankidb::model::*`

- add anki schema definitions for use in `sea-query` builders
- add `prepare` and `prepare_cached` to `Database`
- add query library

## [0.1.1] - 2023-09-15

- add `id_for_deck`, `id_for_notetype`, and `fields_for_notetype` to `Database`
- add `prepare_raw` and `prepare_cached_raw` to `Database`

## [0.1.0] - 2023-09-15

- initial release
