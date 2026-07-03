## [0.3.1](https://github.com/chesteralan/react-auditor/compare/v0.1.11...v0.3.1) (2026-07-03)


# Changelog

## [0.5.0](https://github.com/chesteralan/react-auditor/compare/react-auditor-v0.4.0...react-auditor-v0.5.0) (2026-07-03)


### Features

* 5 new rules, oxc 0.138 migration fixes, docs update ([5eb9eff](https://github.com/chesteralan/react-auditor/commit/5eb9eff536b84e6c32794211bfd93b14d3bd17b0))
* 5 new rules, oxc 0.138 migration fixes, docs update ([34550d7](https://github.com/chesteralan/react-auditor/commit/34550d7c99486b71b5e4dc54a2c3bca7df3af385))
* add --fail-on, colored output with category prefix, Violation.category field ([338ba7c](https://github.com/chesteralan/react-auditor/commit/338ba7cbb2530792598f1ad67220d633c02bdaba))
* add --ignore/--exclude CLI flag for skipping files/directories ([2cef2f2](https://github.com/chesteralan/react-auditor/commit/2cef2f209aa5adf32e0c7202d50400b6a84bc9cb))
* add 4 Next.js rules, 3 fixtures, e2e/unit tests ([b76a0bb](https://github.com/chesteralan/react-auditor/commit/b76a0bbacecb4caaf797ddd9295807ed8a17d0d8))
* add 4 Phase 14 rules (no-direct-mutation, no-ref-in-component-name, no-explicit-any, no-ambiguous-labels) ([affd04a](https://github.com/chesteralan/react-auditor/commit/affd04af87e192b676d91faa2802e8412c4fcc5c))
* add progress indicator during scan ([cd6a431](https://github.com/chesteralan/react-auditor/commit/cd6a43123a66c8098b29b112d31231183644fd92))
* add VS Code extension icon ([cdab9a0](https://github.com/chesteralan/react-auditor/commit/cdab9a0cdc1624241194087a395d87003eb3192f))
* bundle all 5 platform binaries in npm package ([b19f8d1](https://github.com/chesteralan/react-auditor/commit/b19f8d148e6ed1963c304ac4f5949449d1e9a20f))
* initial release — 43 rules, CLI, npm wrapper, CI/CD, benchmarks, VS Code extension ([4584e9b](https://github.com/chesteralan/react-auditor/commit/4584e9be12d4ea461e316ff8b1f77bef21f85711))
* new rules, auto-fix, fuzz harness, testing category ([99051ac](https://github.com/chesteralan/react-auditor/commit/99051acf15f1194ee395ac2c0e992cbb9c57c2bf))
* prefer-function-components auto-fix, assert-includes-message rule, config presets, init cmd, perf ([4b4fa42](https://github.com/chesteralan/react-auditor/commit/4b4fa42582a29871f5828b04396ff0e8609188e3))
* remove push trigger for release-please branch ([0b95fc8](https://github.com/chesteralan/react-auditor/commit/0b95fc80e37d1756afb8da6a1fcab8280c0cdf88))
* remove push trigger for release-please branch ([a7e4684](https://github.com/chesteralan/react-auditor/commit/a7e468429b13b77ae11d912ad6b34931ab0221f2))
* skip npm install if binary already at matching version ([86ba23e](https://github.com/chesteralan/react-auditor/commit/86ba23eaf0412d67ebdb3ea85d968a888dee39a0))
* update npm package version to v0.1.10 ([007c099](https://github.com/chesteralan/react-auditor/commit/007c0997c8aca9e20e50a9279e36ebb41fa3344f))
* wire --rules flag into Scanner, add category filter tests ([101c1fd](https://github.com/chesteralan/react-auditor/commit/101c1fdeca2b18d8ef6efdd1b8e810d4be0358fb))


### Bug Fixes

* accept directory paths as CLI arguments (walk recursively) ([b1c96d5](https://github.com/chesteralan/react-auditor/commit/b1c96d529dee9d7c9196578fd00accab10645f37))
* audit fixes for 8 rule files, rename effect-deps-complete to no-missing-deps, add docs/index.html ([c18b82e](https://github.com/chesteralan/react-auditor/commit/c18b82ebb044481cab5daff0fb6f3d0c686d862d))
* cargo run --version, default-run, help text rule counts ([d1f5586](https://github.com/chesteralan/react-auditor/commit/d1f55862c156fb92f91aae37e4731d970c4addd0))
* install aarch64 cross-compiler in release workflow ([5cc543a](https://github.com/chesteralan/react-auditor/commit/5cc543a78a3fa25879ac176c6908e977fb601262))
* install.js archive URL matching release naming, add tar/gzip extraction ([20f6e9b](https://github.com/chesteralan/react-auditor/commit/20f6e9b69704ff34e3ec1ceab44aeb5d0972b209))
* install.js archive URL matching release naming, add tar/gzip extraction ([7f21971](https://github.com/chesteralan/react-auditor/commit/7f219717e1bd47400ada16cf356609f9135c3ecb))
* npm bin wrapper avoids shell-script recursion ([ce31146](https://github.com/chesteralan/react-auditor/commit/ce3114628572a94c5a0419758fdde31258751e8c))
* npm package no longer downloads binary from GitHub ([4bc8a2c](https://github.com/chesteralan/react-auditor/commit/4bc8a2c022653364279e32a6098a5c344fdf6dd0))
* npm package no longer downloads binary from GitHub ([6ba983f](https://github.com/chesteralan/react-auditor/commit/6ba983f1dd81b8c8c327d9071357806100b1e7e2))
* release-please updates ([9c801a8](https://github.com/chesteralan/react-auditor/commit/9c801a82b9be6d673b8b2077ed515d6346820fa6))
* release-please updates ([9c801a8](https://github.com/chesteralan/react-auditor/commit/9c801a82b9be6d673b8b2077ed515d6346820fa6))
* release-please updates ([84b9dc7](https://github.com/chesteralan/react-auditor/commit/84b9dc7f4f59503570b52bed44fee4e9a191af8a))
* robust version binary path resolution for CI ([a050775](https://github.com/chesteralan/react-auditor/commit/a050775f691e216b222bdcf87a1781544f8b78a3))
* **tests:** make version checks resilient to patch bumps ([a427f8f](https://github.com/chesteralan/react-auditor/commit/a427f8f4edca3fff68c7486a1309bd2746998b17))
* use CARGO_BIN_EXE env var for version test binary path ([e8869b7](https://github.com/chesteralan/react-auditor/commit/e8869b72448e51736c57b60d90fd24bcf60282ee))
* use Node.js wrapper for npm binary ([0697797](https://github.com/chesteralan/react-auditor/commit/0697797e84f9f9876ad01f3ce1619e9b02dc4c55))
* use Node.js wrapper for npm binary ([ebd6108](https://github.com/chesteralan/react-auditor/commit/ebd6108619d576cad3ae81c28b6cbad51af23160))

## [0.4.0](https://github.com/chesteralan/react-auditor/compare/v0.3.1...v0.4.0) (2026-07-03)


### Features

* bundle all 5 platform binaries in npm package ([b19f8d1](https://github.com/chesteralan/react-auditor/commit/b19f8d148e6ed1963c304ac4f5949449d1e9a20f))
* new rules, auto-fix, fuzz harness, testing category ([99051ac](https://github.com/chesteralan/react-auditor/commit/99051acf15f1194ee395ac2c0e992cbb9c57c2bf))
* prefer-function-components auto-fix, assert-includes-message rule, config presets, init cmd, perf ([4b4fa42](https://github.com/chesteralan/react-auditor/commit/4b4fa42582a29871f5828b04396ff0e8609188e3))

## [0.1.11](https://github.com/chesteralan/react-auditor/compare/v0.1.10...v0.1.11) (2026-07-03)


### Bug Fixes

* use Node.js wrapper for npm binary to avoid shim errors ([ebd6108](https://github.com/chesteralan/react-auditor/commit/ebd6108620d7915b3e840da7e08a7f12fe19f85a))


# Changelog

## [0.3.1](https://github.com/chesteralan/react-auditor/compare/v0.3.0...v0.3.1) (2026-07-02)


### Bug Fixes

* use Node.js wrapper for npm binary ([0697797](https://github.com/chesteralan/react-auditor/commit/0697797e84f9f9876ad01f3ce1619e9b02dc4c55))
* use Node.js wrapper for npm binary ([ebd6108](https://github.com/chesteralan/react-auditor/commit/ebd6108619d576cad3ae81c28b6cbad51af23160))

## [0.3.0](https://github.com/chesteralan/react-auditor/compare/v0.2.0...v0.3.0) (2026-07-02)


### Features

* remove push trigger for release-please branch ([0b95fc8](https://github.com/chesteralan/react-auditor/commit/0b95fc80e37d1756afb8da6a1fcab8280c0cdf88))
* remove push trigger for release-please branch ([a7e4684](https://github.com/chesteralan/react-auditor/commit/a7e468429b13b77ae11d912ad6b34931ab0221f2))

## [0.2.0](https://github.com/chesteralan/react-auditor/compare/v0.1.9...v0.2.0) (2026-07-02)


### Features

* update npm package version to v0.1.10 ([007c099](https://github.com/chesteralan/react-auditor/commit/007c0997c8aca9e20e50a9279e36ebb41fa3344f))

## [0.1.9](https://github.com/chesteralan/react-auditor/compare/v0.1.8...v0.1.9) (2026-07-02)


### Bug Fixes

* npm package no longer downloads binary from GitHub ([4bc8a2c](https://github.com/chesteralan/react-auditor/commit/4bc8a2c022653364279e32a6098a5c344fdf6dd0))
* npm package no longer downloads binary from GitHub ([6ba983f](https://github.com/chesteralan/react-auditor/commit/6ba983f1dd81b8c8c327d9071357806100b1e7e2))

## [0.1.8](https://github.com/chesteralan/react-auditor/compare/v0.1.7...v0.1.8) (2026-07-02)


### Bug Fixes

* **tests:** make version checks resilient to patch bumps ([a427f8f](https://github.com/chesteralan/react-auditor/commit/a427f8f4edca3fff68c7486a1309bd2746998b17))

## [0.1.8](https://github.com/chesteralan/react-auditor/compare/v0.1.7...v0.1.8) (2026-07-02)


### Features

* --docs flag generates rule docs in docs/rules/ ([e4d26be](https://github.com/chesteralan/react-auditor/commit/e4d26bed8552810c535183b837a2115faedb9f32))


### Bug Fixes

* npm bin wrapper uses wrapper.js to avoid shell-script recursion ([ce31146](https://github.com/chesteralan/react-auditor/commit/ce31146feacff3de0751956e5cf81e9341fcc605))


## [0.1.7](https://github.com/chesteralan/react-auditor/compare/v0.1.6...v0.1.7) (2026-07-02)


### Bug Fixes

* cargo run --version, default-run, help text rule counts ([d1f5586](https://github.com/chesteralan/react-auditor/commit/d1f55862c156fb92f91aae37e4731d970c4addd0))
* robust version binary path resolution for CI ([a050775](https://github.com/chesteralan/react-auditor/commit/a050775f691e216b222bdcf87a1781544f8b78a3))
* use CARGO_BIN_EXE env var for version test binary path ([e8869b7](https://github.com/chesteralan/react-auditor/commit/e8869b72448e51736c57b60d90fd24bcf60282ee))
