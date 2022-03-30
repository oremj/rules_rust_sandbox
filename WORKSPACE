workspace(name = "rules_rust_sandbox")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

rust_version = "d2e2470cbfb1b8c1cc6dfa4421627d128e9bdc65"

http_archive(
    name = "rules_rust",
    sha256 = "0862a4d88657b3a2a307aefe36addf9dfa8ecbf7a0ec7319b780e43a10a71e28",
    strip_prefix = "rules_rust-%s" % rust_version,
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/%s.tar.gz" % rust_version,
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    include_rustc_srcs = True,
    iso_date = "2022-02-28",
    rustfmt_version = "nightly",
    version = "1.58.1",
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")
crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository")
crates_repository(
    name = "crates_sandbox",
    manifests = [
        "//duplicate_condition:Cargo.toml",
    ],
    lockfile = "//:crate.lock",
    generator = "@cargo_bazel_bootstrap//:cargo-bazel",
    supported_platform_triples = [
        "x86_64-apple-darwin",
        "x86_64-unknown-linux-gnu",
    ],
)

load("@crates_sandbox//:defs.bzl", "crate_repositories")
crate_repositories()