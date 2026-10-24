# norm-adapter-sdk

Shared host-adapter engine SDK for the [norm-spec](https://github.com/CyanoOrg/norm-spec)
family. One implementation of the engine-side merge/projection — protocol
mirrors, single- and multi-target prompt context, layout index — consumed by
every host adapter under an exact pin.

- Host-agnostic: every machine API identifier, error-code prefix, and prompt
  marker is supplied by the adapter's `AdapterIdentity`. Nothing else varies
  between hosts.
- Deterministic: projections preserve upstream ordering, render byte-stable
  output, and fail instead of truncating above their size bounds.
- No hidden scope: no YAML parsing, filesystem walking, schema validation,
  process lifecycle, or network I/O. Those stay with norm-spec and each
  host adapter.

Adapters: [pi-norm-spec](https://github.com/CyanoOrg/pi-norm-spec),
[dsh-norm-spec](https://github.com/CyanoOrg/dsh-norm-spec). The extraction
decision and phase plan are recorded in pi-norm-spec D018 and mirrored here
as D001.

## Status

`0.1.0` pre-release development. The crate is private until the first
crates.io publication review. See `docs/planning/status.md`.

## License

MIT.
