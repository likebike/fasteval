# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
(Click the above link to see the work that has occurred since the latest release.)

## [0.2.3] - 2020-01-16


## [0.2.2] - 2020-01-15
### Removed
- Removed the `fasteval::parse()` convenience function.  Now use the `Parser`
  directly: `Parser::new().parse()`.  This was done to improve the usability of
  custom safety parse limits.

## [0.2.1] - 2020-01-14
### Added
- Enable custom safety parse limits.
- Better documentation of safety features.

## [0.2.0] - 2020-01-01
This was the initial public release.  Changes before this point are not
described here, but they can still be viewed in the [Repository].

[Unreleased]: https://github.com/likebike/fasteval/compare/0.2.3...HEAD
[0.2.3]: https://github.com/likebike/fasteval/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/likebike/fasteval/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/likebike/fasteval/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/likebike/fasteval/releases/tag/0.2.0
[Repository]: https://github.com/likebike/fasteval

