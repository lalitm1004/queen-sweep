# Releasing the Extension

Extension builds are published as GitHub Releases. The `Release Extension`
workflow builds the extension, packages `dist/`, and attaches the archive to a
release.

## Versioning

The version is declared in two files and must match:

- `queen-sweep-extension/package.json` (`version`)
- `queen-sweep-extension/src/manifest.json` (`version`)

Chrome reads the manifest version, so it is the source of truth for a release.
The workflow reads it from `dist/manifest.json` after building and checks it
against the tag.

## Cut a release

1. Set both version files to the new version.
2. Commit and push to `main`.
3. Tag and push the tag:

```sh
git tag extension-v0.1.0
git push origin extension-v0.1.0
```

The tag must be `extension-v<version>`. If it does not match the manifest
version, the workflow fails before publishing.

## Workflow

`.github/workflows/release-extension.yml` runs on tags matching `extension-v*`
and:

1. installs Rust, the `wasm32-unknown-unknown` target, `wasm-pack`, `binaryen`,
   and Node.js
2. runs `npm ci` and `npm run build` in `queen-sweep-extension`
3. checks the tag against the manifest version
4. packages `dist/` as `queen-sweep-extension-v<version>/` and zips it to
   `queen-sweep-extension-v<version>.zip`
5. creates the GitHub Release with the zip attached

## Manual run

Start the workflow from the Actions tab, or with the CLI:

```sh
gh workflow run "Release Extension" -f tag=extension-v0.1.0
```

Manual runs skip the tag/version check.

## Verify

```sh
gh run list --workflow "Release Extension"
gh release view extension-v0.1.0
```

The downloadable asset is `queen-sweep-extension-v<version>.zip`. Users unzip
it and load the folder as an unpacked extension. GitHub's generated source
archives contain the repository source, not the built extension.

## Notes

- The same archive can be uploaded to the Chrome Web Store.
- The solver supports board sizes `1` and `4..=18`.
