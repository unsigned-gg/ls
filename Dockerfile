# Runtime image for the static musl lsc binary built by `cargo xtask package`.
# Base is pinned by digest (gcr.io/distroless/static:nonroot, resolved 2026-10-12).
FROM gcr.io/distroless/static@sha256:e2e927ec666bae08560abb3c55d0659eceabb657f56b6782ab500a9fc7f555e3
LABEL org.opencontainers.image.source="https://github.com/unsigned-gg/ls" \
      org.opencontainers.image.description="lsc: LeanSpec parser, structural checker and CAS listing engine" \
      org.opencontainers.image.licenses="Apache-2.0 OR MIT OR CC0-1.0"
COPY dist/lsc /lsc
USER nonroot
ENTRYPOINT ["/lsc"]
