# Metaplex Core Utilities

Core Rust utilities for Metaplex smart contracts.

[![Program Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml)
[![Integration Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml)
[![SDK Tests](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml/badge.svg)](https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml)

## Metaplex Core Utilities

| Name          | Rust Crate                                                 | npm Package |
| :------------ | :--------------------------------------------------------- | ----------- |
| Testing Utils | [![Crate][mpl-testing-utils-img]][mpl-testing-utils-crate] | N/A         |
| Utils         | [![Crate][mpl-utils-img]][mpl-utils-crate]                 | N/A         |

## Development

### Setting up Rust Tests

Run the `build.sh` script with "testing-utils" to build the shared object and put it in a directory
called `test-programs` in the root of the project.

```bash
./build.sh testing-utils
```

Similarly, use `test.sh` to run tests:

```bash
./test.sh testing-utils
```

### Versioning and Publishing Packages

We use the following `(pre|post)(version|publish)` npm scripts to manage related checks, tagging,
committing and pushing the version bump.

- `preversion`: ensures that the package builds and its tests pass
- `postversion`: adds and commits the version bump and adds a tag indicating package name and new
  version, i.e. `@metaplex-foundation/mp-core@v0.0.1`
- `prepublishOnly`: ensures that the package builds and its tests pass again (just to be _really_
  sure)
- `postpublish`: pushes the committed change and new tag to GitHub

In order to version and then publish a package just run the following commands from the folder of
the package you want to update:

- `npm version <patch|minor|major>`
- `npm publish`

As you note if version + publish succeeds the scripts end up pushing those updates to the master
branch. Therefore, please ensure to be on and up to date `master` branch before running them. Please
**don't ever publish from another branch** but only from the main one with only PR approved changes
merged.

### Rust Crates

| Package       | Link                                         | Version                                                    |
| :------------ | :------------------------------------------- | :--------------------------------------------------------- |
| Testing Utils | [mpl-testing-utils][mpl-testing-utils-crate] | [![Crate][mpl-testing-utils-img]][mpl-testing-utils-crate] |
| Utils         | [mpl-utils][mpl-utils-crate]                 | [![Crate][mpl-utils-img]][mpl-utils-crate]                 |

## Reporting security issues

To report a security issue, please follow the guidance on the [SECURITY](.github/SECURITY.md) page.

## License

The Rust/Cargo programs are licensed under the "Apache-style" [Metaplex(TM) NFT Open Source
License][metaplex-nft-license] and the JS/TS client libraries are licensed under either the
[MIT][mit-license] or the [Apache][apache-license] licenses.

<!-- ===================================== -->
<!-- Links for badges and such shown above -->
<!-- Please add any links you add to the   -->
<!-- readme here instead of inlining them  -->
<!-- ===================================== -->

<!-- Workflow Status Badges -->

[integration-tests-yml]:
  https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml
[integration-tests-svg]:
  https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/integration.yml/badge.svg
[program-tests-yml]:
  https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml
[program-tests-svg]:
  https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/program.yml/badge.svg
[sdk-tests-yml]:
  https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml
[sdk-tests-svg]:
  https://github.com/metaplex-foundation/metaplex-program-library/actions/workflows/sdk.yml/badge.svg

<!-- Crates -->

[mpl-utils-crate]: https://crates.io/crates/mpl-utils
[mpl-testing-utils-crate]: https://crates.io/crates/mpl-testing-utils
[mpl-utils-img-long]:
  https://img.shields.io/crates/v/mpl-utils?label=crates.io%20%7C%20mpl-utils&logo=rust
[mpl-utils-img]: https://img.shields.io/crates/v/mpl-utils?logo=rust
[mpl-testing-utils-img-long]:
  https://img.shields.io/crates/v/mpl-testing-utils?label=crates.io%20%7C%20mpl-testing-utils&logo=rust
[mpl-testing-utils-img]: https://img.shields.io/crates/v/mpl-testing-utils?logo=rust

<!-- Licenses -->

[metaplex-nft-license]:
  https://github.com/metaplex-foundation/metaplex-program-library/blob/master/LICENSE
[apache-license]: https://www.apache.org/licenses/LICENSE-2.0.txt
[mit-license]: https://www.mit.edu/~amini/LICENSE.md
