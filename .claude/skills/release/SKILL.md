---
name: release
description: Cut a versioned release of ARKONK and upload it to Steam. Run by the owner only.
disable-model-invocation: true
---

# Release

Follow `docs/RELEASING.md` step by step. It is the source of truth for versioning,
signing, notarization and the Steam upload. In short:

1. `main` is green and contains everything for the release.
2. Do the version bump and the `CHANGELOG.md` section on a `chore/release-x.y.z`
   branch, through a PR like any other change.
3. After it merges, tag the merged commit:
   `git tag -a vX.Y.Z -m "ARKONK X.Y.Z" origin/main && git push origin vX.Y.Z`.
   The tag must match `Cargo.toml`.
4. Watch the Release workflow. It packages and signs every platform, publishes the
   GitHub Release, and uploads to Steam when the Steam secrets are set.
5. Set the build live on the Steam branch named in `docs/RELEASING.md`, then smoke-test
   the Steam install on each platform.
