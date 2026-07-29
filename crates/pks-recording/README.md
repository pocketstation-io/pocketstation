# pks-recording

Concrete multistem WAV recording and endpoint finalization for PocketStation.

This package implements recording behind the generic `pks-endpoint` lifecycle.
It does not own Session lifecycle, graph compilation, runtime scheduling,
capture, providers, or the public SDK.
