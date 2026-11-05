# Release the npm packages and database

Use this guide to publish the six npm packages and the matching database assets. One project release on GitHub contains changelog notes and npm links, not server binaries.

## Prerequisites

Run release commands from the repository root in Bash.

- Use a checkout of `main` and an authenticated GitHub CLI (`gh`).
- Use Node.js 24 and pnpm 10.34.9. The root `package.json` pins pnpm.
- Install the workspace tools without running package scripts:

  ```sh
  pnpm install --frozen-lockfile --ignore-scripts
  ```

The version workflow needs permission to open pull requests. Under **Settings > Actions > General**, enable **Allow GitHub Actions to create and approve pull requests** before merging a changeset. See [GitHub Actions settings](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#preventing-github-actions-from-creating-or-approving-pull-requests).

## Release the database

Release binaries embed the checksum from the immutable database release for their schema.

The database release must match `SCHEMA_VERSION` in `src/schema_version.rs`. Its tag is `db-schemaN`.
The release must contain `wotlk-db-schemaN.sqlite.zst` and `wotlk-db-schemaN.sqlite.sha256`. Here, `N` is the schema number.
The checksum covers the decompressed SQLite file. After the `Release` workflow verifies a push to `main`, it checks database readiness in parallel with Changesets preparation.
Native builds wait for database readiness. This check runs even when no npm release is pending.

The workflow skips publication when both assets match the tracked database. If the release is absent or has neither expected asset, it publishes them.
Dispatch the database workflow from `main` only for repair or bootstrap:

```sh
gh workflow run release-database.yml --ref main
```

Wait for the `Release database` workflow to succeed. Database tags and assets are immutable.
If the workflow finds one expected asset, a malformed checksum, or a different checksum, increase `SCHEMA_VERSION` and publish a new schema. Do not replace existing assets.

## Prepare a release

Changesets versions the launcher and five platform packages as one fixed group.

| Package | Platform |
| --- | --- |
| `wotlk-database-mcp` | Launcher |
| `wotlk-database-mcp-darwin-arm64` | macOS ARM64 |
| `wotlk-database-mcp-darwin-x64` | macOS x86_64 |
| `wotlk-database-mcp-linux-arm64` | Linux ARM64 with glibc |
| `wotlk-database-mcp-linux-x64` | Linux x86_64 with glibc |
| `wotlk-database-mcp-win32-x64` | Windows x86_64 |

Every release publishes all six packages at the same version, including packages with no direct changes. The Cargo server version follows the launcher version.

### Add a changeset

Create one changeset for a user-visible package change. Changesets asks which package changed, the version bump, and a summary for its package changelog.

```sh
pnpm exec changeset
pnpm exec changeset status
```

Choose the change's release level. The fixed group gives all six packages the same version. Review the changeset summary before merging your change to `main`.
See the [Changesets CLI guide](https://changesets.dev/guide/cli) for command details.

### Merge the version pull request

When a changeset reaches `main`, the `Release` workflow opens or updates a version pull request after the CI checks pass.
If GitHub marks the bot-created or updated checks as approval-required, a repository writer must select **Approve workflows to run**.
Review the checks, all six package versions, and their generated changelog entries. The version script also synchronizes the Cargo version and lockfile.
Merge the version pull request to start the release workflow.

The `Release` workflow runs the repository's CI checks once. After CI succeeds, it prepares Changesets and checks database readiness in parallel.
For a publish, native builds wait for database readiness and smoke-test binaries for all five platforms. The workflow packs the packages with pnpm only after every build and smoke check passes.
The workflow creates one Git tag and notes-only GitHub release named `v<version>` for all six packages. The GitHub release has no binary attachments.
The workflow keeps raw native binaries as Actions artifacts for one day.

## Publish the first npm versions

Publish real packages before configuring npm trusted publishers. Do not publish placeholder packages. A new package must exist on npm before its trusted publisher can be configured.

After the first version pull request is merged, wait for the release run to build all five native binaries. New packages cannot use trusted publishing for their first publish.
Use a clean checkout of the exact commit shown in that run. Download the raw artifacts before their one-day retention expires.
Run the commands in a live interactive terminal so you can complete npm's browser and passkey approval. If npm offers a five-minute approval window, choose it in npm's prompt. Do not create or use an npm token.

```sh
set -euo pipefail
npm login
read -r -p 'Release run ID: ' RUN_ID
ARTIFACT_DIR=$(mktemp -d)
gh run download "$RUN_ID" --pattern 'native-*' --dir "$ARTIFACT_DIR"
node npm/build.mjs \
  "darwin-arm64=$ARTIFACT_DIR/native-darwin-arm64/wotlk-database-mcp" \
  "darwin-x64=$ARTIFACT_DIR/native-darwin-x64/wotlk-database-mcp" \
  "linux-arm64=$ARTIFACT_DIR/native-linux-arm64/wotlk-database-mcp" \
  "linux-x64=$ARTIFACT_DIR/native-linux-x64/wotlk-database-mcp" \
  "win32-x64=$ARTIFACT_DIR/native-win32-x64/wotlk-database-mcp.exe"
pnpm exec changeset publish --git-tag false
```

Enter the workflow run ID shown in its GitHub Actions URL when prompted. `npm login` must use an account allowed to publish all six public package names.

`npm/build.mjs` stages the downloaded binaries and the MIT license in the platform packages. `pnpm exec changeset publish --git-tag false` packs the workspaces with pnpm without creating package tags.
pnpm resolves their `workspace:*` dependencies to the release versions. Do not use `npm pack` on these workspace packages.

After all six versions are published, create the project release if the workflow did not create it. Copy the section for this version from `npm/CHANGELOG.md` into a notes file. Add links to all six npm packages listed above, including the launcher and its five optional platform dependencies. Choose the notes file path when prompted below.

Verify that the checkout is the source commit for the release run. If the project tag already exists, do not create or move it. Check that it points to that commit before creating a missing release.

```sh
VERSION=$(node -p "require('./npm/package.json').version")
COMMIT=$(git rev-parse HEAD)
RUN_COMMIT=$(gh run view "$RUN_ID" --json headSha --jq .headSha)
test "$COMMIT" = "$RUN_COMMIT" || { echo "Checkout does not match release run" >&2; exit 1; }
read -r -p 'Release notes file path: ' NOTES_FILE
test -f "$NOTES_FILE" || { echo "Notes file not found" >&2; exit 1; }
git tag "v$VERSION" "$COMMIT"
git push origin "refs/tags/v$VERSION"
gh release create "v$VERSION" --verify-tag --title "v$VERSION" --notes-file "$NOTES_FILE"
```

This command creates no release assets. Existing legacy `package@version` tags may remain, but do not create new package tags or package releases.

## Configure trusted publishing

Configure each npm package after its first real version is published. Do this shortly before the next release.
On npmjs.com, open each package's **Settings > Trusted Publisher** section and select **GitHub Actions**. npm requires a successful publish within two days of creating the configuration.

| npm setting | Value |
| --- | --- |
| Organization or user | `denisdasilvarocha` |
| Repository | `wotlk-database-mcp` |
| Workflow filename | `release.yml` |
| Environment name | Leave blank |
| Allowed action | Allow `npm publish` |

The release workflow uses `id-token: write` in its publish job and does not use a GitHub environment. Keep the workflow filename `release.yml`.
Trusted publishing requires npm CLI 11.5.1 or later and Node.js 22.14.0 or later. The release workflow uses Node.js 24.
See [npm trusted publishers](https://docs.npmjs.com/trusted-publishers/) for the setup requirements.

## Verify the release

Check that npm contains the released version of every package:

```sh
VERSION=$(node -p "require('./npm/package.json').version")
for name in \
  wotlk-database-mcp \
  wotlk-database-mcp-darwin-arm64 \
  wotlk-database-mcp-darwin-x64 \
  wotlk-database-mcp-linux-arm64 \
  wotlk-database-mcp-linux-x64 \
  wotlk-database-mcp-win32-x64
do
  npm view "$name@$VERSION" version
done
npm view wotlk-database-mcp dist-tags
gh release view "v$VERSION"
```

Install the published launcher in a clean directory, then run the protocol smoke:

```sh
VERSION=$(node -p "require('./npm/package.json').version")
SMOKE_DIR=$(mktemp -d)
printf '{"private":true}\n' > "$SMOKE_DIR/package.json"
npm install --prefix "$SMOKE_DIR" --ignore-scripts "wotlk-database-mcp@$VERSION"
node npm/smoke.mjs "$SMOKE_DIR"
```

The smoke starts the installed launcher through `npx` and sends MCP `initialize` and `tools/list` requests. It verifies protocol behavior. The handshake and tool list work without a database download.

## Recover from a partial publish

> [!CAUTION]
> npm does not allow a package name and version to be published twice. Never delete, move, force-push, or recreate a project tag, package tag, or immutable database tag.

For an authentication or network failure, check each exact `package@version` in the registry. Fix the cause and rerun the failed publish job from the same workflow run.
Changesets skips versions that already exist on npm and publishes only missing package versions. If code or package contents must change, add a changeset and release a new version.

After the retry, check whether the `v<version>` project tag and release exist. Preserve all existing tags. If the project release is missing and its tag exists, verify that the tag points to the exact release run commit. Set `VERSION` from `npm/package.json` and `NOTES_FILE` to your notes file path. Use the launcher changelog section and six npm package links as described above. Then create the missing release:

```sh
VERSION=$(node -p "require('./npm/package.json').version")
read -r -p 'Release notes file path: ' NOTES_FILE
gh release create "v$VERSION" --verify-tag --title "v$VERSION" --notes-file "$NOTES_FILE"
```

If both tag and release are missing, follow the project release steps above. Do not create package releases or attach native binaries.

## Related pages

- [Install for an MCP harness](../install.md) to run the npm package.
- [Rebuild the database](rebuild-database.md) before changing database inputs.
- [Changesets CLI guide](https://changesets.dev/guide/cli) for version and publish commands.
- [npm trusted publishers](https://docs.npmjs.com/trusted-publishers/) for OIDC publishing requirements.
