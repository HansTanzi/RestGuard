# Security policy

## Supported versions

Only the latest release gets security fixes. Please update before reporting.

## Reporting a vulnerability

Please **don't** open a public issue for security problems.

Report it privately through GitHub instead: go to the [Security tab](https://github.com/HansTanzi/RestGuard/security) and click **Report a vulnerability** ([direct link](https://github.com/HansTanzi/RestGuard/security/advisories/new)). Only the maintainer can see the report.

Please include:

- What the problem is and what an attacker could do with it
- Steps to reproduce, or a proof of concept
- Your OS and RestGuard version

You'll get a reply within 7 days. Once a fix is released, the advisory is published, with credit to you if you'd like.

## Scope

Things that are in scope include, for example:

- Release files that don't match their source, `SHA256SUMS.txt` or build provenance attestation
- Problems in the Scoop bucket, Homebrew tap, APT repository or WinGet manifest that could deliver something other than the official build
- The ntfy phone notification leaking more than the notification text, or sending to a server other than the configured one
- A local process abusing RestGuard to gain privileges it wouldn't otherwise have

Ways to get around a break (e.g. Force Quit on macOS, or ending the process in Task Manager) are expected and not security issues; see [Platform limitations](README.md#platform-limitations). Feel free to open a normal issue if you think one should be blocked.
