# Changelog

## [NEXT]

- _breaking_: add sea-query and sea-query-rusqlite dependencies
- _breaking_: stop re-exporting id structs and instead expose ankidb::model::\*

- add anki schema definitions for use in sea-query builders
- add prepare and prepare_cached to Database

## [0.1.1] - 2023-09-15

- add id_for_deck, id_for_notetype, and fields_for_notetype to Database
- add prepare_raw and prepare_cached_raw to Database

## [0.1.0] - 2023-09-15

- initial release
