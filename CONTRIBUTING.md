# Introduction

First off, thank you for considering contributing to "Onion Or Not The Onion Drinking Game 2".
It's my first real open project and I am really happy that you are interested in contributing.
That warms my heart. <3

Following these guidelines helps to communicate that you respect the time of the developer managing and developing this open source project, which is me.
In return, I should reciprocate that respect in addressing your issue, assessing changes, and helping you finalize your pull requests.

"Onion Or Not The Onion Drinking Game 2" is an open source project and I love to receive contributions from our community — you!
There are many ways to contribute, from writing tutorials or blog posts, improving the documentation, submitting bug reports and feature requests or writing code which can be incorporated into "Onion Or Not The Onion Drinking Game 2" itself.

This little project of mine currently only has one main developer, which is me.
I am working full time and this is only a side project.
So please don't overflow me with requests and be patient if I need time to answer or don't have the time to react instantly.

# Ground Rules

Responsibilities:
* Ensure that with any code change you provide all projects still
  * compile (`cargo build`),
  * the tests pass (`cargo test`),
  * the code is formatted (`cargo fmt`),
  * there are no lint errors nor warnings (`cargo clippy -- -D warnings`)
  * and no unknown security vulnerabilities are found (`cargo deny check advisories`)
  * *(for advanced users see GitHub Action in `.github/workflows/general.yml` and/or useful git hook in `githooks/pre-commit.lints`)*.
* Try to produce idiomatic *(for example [Idiomatic Rust](https://cheats.rs/#idiomatic-rust))* and clean code.
* Add automatic testing *(unit tests, integration tests)* where needed and reasonable.
* Ensure cross-platform compatibility for every change *(For example don't use platform specific code with `cfg(...)`if not really needed)*.
* Create issues for any major changes and enhancements that you wish to make. Discuss things transparently and get community feedback.
* Be welcoming to newcomers and encourage diverse new contributors from all backgrounds. See the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

# Your First Contribution

Unsure where to begin contributing to "Onion Or Not The Onion Drinking Game 2"?

You can start by looking through beginner and help-wanted issues, if there are any:
- Beginner issues - issues which should only require a few lines of code, and a test or two.
- Help wanted issues - issues which should be a bit more involved than beginner issues.

Working on your first Pull Request?
Here are a couple of friendly tutorials: [http://makeapullrequest.com/](http://makeapullrequest.com/) and [http://www.firsttimersonly.com/](http://www.firsttimersonly.com/)

At this point, you're ready to make your changes!
Feel free to ask for help; everyone is a beginner at first :smile_cat:

If a maintainer asks you to "rebase" your PR, they're saying that a lot of code has changed, and that you need to update your branch so it's easier to merge.

# Getting started

**Attention:** Whenever you contribute to this project, you automatically agree to provide your work under the same license as this project is provided under.
See the `LICENSE` file.

For any change you want to propose:
1. Create your own fork of the code
2. Do the changes in your fork
3. If you like the change and think the project could use it:
  * Try to produce idiomatic *(for example [Idiomatic Rust](https://cheats.rs/#idiomatic-rust))* and clean code
  * Add automatic testing *(unit tests, integration tests)* where needed and reasonable
  * Ensure cross-platform compatibility for every change *(For example don't use platform specific code with `cfg(...)`if not really needed)*
  * Be sure that all projects still
	  * compile (`cargo build`),
	  * the tests pass (`cargo test`),
	  * the code is formatted (`cargo fmt`),
	  * there are no lint errors nor warnings (`cargo clippy -- -D warnings`)
	  * and no unknown security vulnerabilities are found (`cargo deny check advisories`)
  * Note that any change you propose will be accepted under the same license as this project is provided under (See `LICENSE` file)

# How to report a bug

If you find a security vulnerability, do NOT open an issue.
Email tiquthon@gmail.com instead.

In order to determine whether you are dealing with a security issue, ask yourself these two questions:
* Can I access something that's not mine, or something I shouldn't have access to?
* Can I disable something for other people?

If the answer to either of those two questions are "yes", then you're probably dealing with a security issue.
Note that even if you answer "no" to both questions, you may still be dealing with a security issue, so if you're unsure, just email us.

When filing an issue, make sure to answer these five questions:

1. What version of this project are you using (most likely commit hash)?
2. What operating system and processor architecture are you using?
3. What did you do?
4. What did you expect to see?
5. What did you see instead?

# How to suggest a feature or enhancement

If you find yourself wishing for a feature that doesn't exist in "Onion Or Not The Onion Drinking Game 2", you are probably not alone.
There are bound to be others out there with similar needs.
Open an issue on our issues list on GitHub which describes the feature you would like to see, why you need it, and how it should work.

# Code review process

This is a private project currently only maintained by one developer, which is me.
If the pull request is good enough *(good explanation on why and how, good code quality)* I may just accept it.
If I have any questions I will go into discussion with you and maybe ask you to improve some parts.
I won't add commits to your pull request, only if you directly ask me to do that and am able to do.

---

CONTRIBUTING created with the help of [https://github.com/nayafia/contributing-template/CONTRIBUTING-template.md](https://github.com/nayafia/contributing-template/blob/ab3044b0b5812708e1d561815ce6b9dd53e1d6ae/CONTRIBUTING-template.md)
