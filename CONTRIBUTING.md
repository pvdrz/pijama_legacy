# Contributing to Pijama

Pijama is a project to learn about programming languages and type systems. If
you have some interest in those topics regardless of your experience level
you're welcome to contribute.

Here are some of the different ways of contributing to Pijama.

## Feature Requests

If you would like to see a particular feature in the language (regardless of
whether you want to implement or not) you can open a new issue to discuss it.
This is a young project and there is room for a lot of improvements.

Before opening the issue, look for similar issues with the
[C-enhancement](https://github.com/christianpoveda/pijama/labels/C-enhancement)
label.

## Bug Reports

If you've written a program in Pijama and notice any unexpected or weird
behaviour, please report it in a new issue. We have a test/benchmark suite but
it is far from having good coverage.

Before opening the issue, look for similar issues with the
[C-bug](https://github.com/christianpoveda/pijama/labels/C-bug) label.

## Pull Requests

If you want to hack on Pijama, you can open a pull request against the master
branch of this repository. We use the fork and pull model to handle
contributions.

### First Pull Request

If you've never contributed to a programming language or Rust project, this
project is a good way to get your feet wet. Look for issues with the
[E-easy](https://github.com/christianpoveda/pijama/labels/E-easy) or
[E-mentoring](https://github.com/christianpoveda/pijama/labels/E-mentoring)
labels.

### General Tips

If your pull request addresses an issue from the tracker don't forget to
include a closing keyword (such as `fixes #123`) in the description of the pull
request. This helps to keep the tracker clean and up to date.

Before deciding to take an issue, add a comment stating that you will take care
of that issue so we can assign it to you and avoid duplicated work.

Be sure to run the test suite with `cargo test` before submitting a new pull
request. If your changes modify the language evaluation in any way, run the
benchmarks with `cargo bench` and publish your results in the pull request.

Pijama compiles with Rust stable without any problems and it is our default target.
However we use some nightly features of `rustfmt` so to format your code you
need to run

```bash
cargo +nightly fmt
```
