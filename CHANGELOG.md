# Changelog

## 0.4.0 -- Jun 2022 (unreleased)

- Support asking the AMS router for an open port.  This is required
  when connecting to a PLC on 127.0.0.1.
- Add more known ADS states.
- Document the `symbol::Symbol` and `symbol::Type` flags.
- Add an adstool command to list AMS routes on the target.

## 0.3.1 -- May 2022

- Fix Rust 1.48 compatibility.

## 0.3.0 -- May 2022

- Add `symbol::get_symbol_info()` and related structs.
- Add an adstool option to automatically set a route.
- Add an adstool subcommand to query module licenses.
- Add an adstool command to query the target description.

## 0.1.1 -- Nov 2021

- Add `Client::source()`.
- Add `Client::write_read_exact()`.
- Add `symbol::get_size()` and `symbol::get_location()`.
- Add more known index groups.
- Support system info from TC/RTOS.
- Display ADS errors in hex.
- Many improvements to the adstool example.

## 0.1.0 -- Nov 2021

- Initial release.