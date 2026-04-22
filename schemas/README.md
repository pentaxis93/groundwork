# Schemas

This directory contains the JSON Schemas that groundwork exposes at runtime.
Most of these schemas are methodology-private and are defined only in this
repository.

`request.schema.json` is different: it is a vendored copy of the canonical
request contract maintained by `tesserine/commons`. Groundwork keeps the runtime
copy here so runtime consumers still read schemas from groundwork, not from
commons.

The vendored request schema carries provenance metadata identifying the
canonical authority, an immutable release-tag or commit-SHA URL for the
canonical schema and prose, and the request spec's full semver. When updating
the vendored copy, update both the schema content and the provenance metadata
together so conformance stays explicit.
