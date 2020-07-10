cargo init (Create rust project)

Why Cargo Exists
Cargo is a tool that allows Rust packages to declare their various dependencies and ensure that you’ll always get a repeatable build.

To accomplish this goal, Cargo does four things:

Introduces two metadata files with various bits of package information.
Fetches and builds your package’s dependencies.
Invokes rustc or another build tool with the correct parameters to build your package.
Introduces conventions to make working with Rust packages easier.

Creating a New Package
To start a new package with Cargo, use cargo new:


$ cargo new hello_world --bin
We’re passing --bin because we’re making a binary program: if we were making a library, we’d pass --lib. This also initializes a new git repository by default. If you don't want it to do that, pass --vcs none.

Let’s check out what Cargo has generated for us:


$ cd hello_world
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
Let’s take a closer look at Cargo.toml:


[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]

This is called a manifest, and it contains all of the metadata that Cargo needs to compile your package.

Here’s what’s in src/main.rs:


fn main() {
    println!("Hello, world!");
}
Cargo generated a “hello world” for us. Let’s compile it:


$ cargo build
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
And then run it:


$ ./target/debug/hello_world
Hello, world!
We can also use cargo run to compile and then run it, all in one step (You won't see the Compiling line if you have not made any changes since you last compiled):


$ cargo run
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
     Running `target/debug/hello_world`
Hello, world!
You’ll now notice a new file, Cargo.lock. It contains information about our dependencies. Since we don’t have any yet, it’s not very interesting.

Once you’re ready for release, you can use cargo build --release to compile your files with optimizations turned on:


$ cargo build --release
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
cargo build --release puts the resulting binary in target/release instead of target/debug.

Compiling in debug mode is the default for development. Compilation time is shorter since the compiler doesn't do optimizations, but the code will run slower. Release mode takes longer to compile, but the code will run faster.

Working on an Existing Cargo Package
If you download an existing package that uses Cargo, it’s really easy to get going.

First, get the package from somewhere. In this example, we’ll use rand cloned from its repository on GitHub:


$ git clone https://github.com/rust-lang-nursery/rand.git
$ cd rand
To build, use cargo build:


$ cargo build
   Compiling rand v0.1.0 (file:///path/to/package/rand)
This will fetch all of the dependencies and then build them, along with the package.

Dependencies
crates.io is the Rust community's central package registry that serves as a location to discover and download packages. cargo is configured to use it by default to find requested packages.

To depend on a library hosted on crates.io, add it to your Cargo.toml.

Adding a dependency
If your Cargo.toml doesn't already have a [dependencies] section, add that, then list the crate name and version that you would like to use. This example adds a dependency of the time crate:


[dependencies]
time = "0.1.12"
The version string is a semver version requirement. The specifying dependencies docs have more information about the options you have here.

If we also wanted to add a dependency on the regex crate, we would not need to add [dependencies] for each crate listed. Here's what your whole Cargo.toml file would look like with dependencies on the time and regex crates:


[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]
time = "0.1.12"
regex = "0.1.41"
Re-run cargo build, and Cargo will fetch the new dependencies and all of their dependencies, compile them all, and update the Cargo.lock:


$ cargo build
      Updating crates.io index
   Downloading memchr v0.1.5
   Downloading libc v0.1.10
   Downloading regex-syntax v0.2.1
   Downloading memchr v0.1.5
   Downloading aho-corasick v0.3.0
   Downloading regex v0.1.41
     Compiling memchr v0.1.5
     Compiling libc v0.1.10
     Compiling regex-syntax v0.2.1
     Compiling memchr v0.1.5
     Compiling aho-corasick v0.3.0
     Compiling regex v0.1.41
     Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
Our Cargo.lock contains the exact information about which revision of all of these dependencies we used.

Now, if regex gets updated, we will still build with the same revision until we choose to cargo update.

You can now use the regex library in main.rs.


use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));
}
Running it will show:


$ cargo run
   Running `target/hello_world`
Did our date match? true

Package Layout
Cargo uses conventions for file placement to make it easy to dive into a new Cargo package:


.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
Cargo.toml and Cargo.lock are stored in the root of your package (package root).
Source code goes in the src directory.
The default library file is src/lib.rs.
The default executable file is src/main.rs.
Other executables can be placed in src/bin/.
Benchmarks go in the benches directory.
Examples go in the examples directory.
Integration tests go in the tests directory.
If a binary, example, bench, or integration test consists of multiple source files, place a main.rs file along with the extra modules within a subdirectory of the src/bin, examples, benches, or tests directory. The name of the executable will be the directory name.

You can learn more about Rust's module system in the book.

See Configuring a target for more details on manually configuring targets. See Target auto-discovery for more information on controlling how Cargo automatically infers target names.

Cargo.toml vs Cargo.lock
Cargo.toml and Cargo.lock serve two different purposes. Before we talk about them, here’s a summary:

Cargo.toml is about describing your dependencies in a broad sense, and is written by you.
Cargo.lock contains exact information about your dependencies. It is maintained by Cargo and should not be manually edited.
If you’re building a non-end product, such as a rust library that other rust packages will depend on, put Cargo.lock in your .gitignore. If you’re building an end product, which are executable like command-line tool or an application, or a system library with crate-type of staticlib or cdylib, check Cargo.lock into git. If you're curious about why that is, see "Why do binaries have Cargo.lock in version control, but not libraries?" in the FAQ.

Let’s dig in a little bit more.

Cargo.toml is a manifest file in which we can specify a bunch of different metadata about our package. For example, we can say that we depend on another package:


[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
This package has a single dependency, on the rand library. We’ve stated in this case that we’re relying on a particular Git repository that lives on GitHub. Since we haven’t specified any other information, Cargo assumes that we intend to use the latest commit on the master branch to build our package.

Sound good? Well, there’s one problem: If you build this package today, and then you send a copy to me, and I build this package tomorrow, something bad could happen. There could be more commits to rand in the meantime, and my build would include new commits while yours would not. Therefore, we would get different builds. This would be bad because we want reproducible builds.

We could fix this problem by putting a rev line in our Cargo.toml:


[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev = "9f35b8e" }
Now our builds will be the same. But there’s a big drawback: now we have to manually think about SHA-1s every time we want to update our library. This is both tedious and error prone.

Enter the Cargo.lock. Because of its existence, we don’t need to manually keep track of the exact revisions: Cargo will do it for us. When we have a manifest like this:


[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
Cargo will take the latest commit and write that information out into our Cargo.lock when we build for the first time. That file will look like this:


[[package]]
name = "hello_world"
version = "0.1.0"
dependencies = [
 "rand 0.1.0 (git+https://github.com/rust-lang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9)",
]

[[package]]
name = "rand"
version = "0.1.0"
source = "git+https://github.com/rust-lang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9"
You can see that there’s a lot more information here, including the exact revision we used to build. Now when you give your package to someone else, they’ll use the exact same SHA, even though we didn’t specify it in our Cargo.toml.

When we’re ready to opt in to a new version of the library, Cargo can re-calculate the dependencies and update things for us:


$ cargo update           # updates all dependencies
$ cargo update -p rand   # updates just “rand”
This will write out a new Cargo.lock with the new version information. Note that the argument to cargo update is actually a Package ID Specification and rand is just a short specification.

Tests
Cargo can run your tests with the cargo test command. Cargo looks for tests to run in two places: in each of your src files and any tests in tests/. Tests in your src files should be unit tests, and tests in tests/ should be integration-style tests. As such, you’ll need to import your crates into the files in tests.

Here's an example of running cargo test in our package, which currently has no tests:


$ cargo test
   Compiling rand v0.1.0 (https://github.com/rust-lang-nursery/rand.git#9f35b8e)
   Compiling hello_world v0.1.0 (file:///path/to/package/hello_world)
     Running target/test/hello_world-9c2b65bbb79eabce

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
If our package had tests, we would see more output with the correct number of tests.

You can also run a specific test by passing a filter:


$ cargo test foo
This will run any test with foo in its name.

cargo test runs additional checks as well. For example, it will compile any examples you’ve included and will also test the examples in your documentation. Please see the testing guide in the Rust documentation for more details.

Continuous Integration
Travis CI
To test your package on Travis CI, here is a sample .travis.yml file:


language: rust
rust:
  - stable
  - beta
  - nightly
matrix:
  allow_failures:
    - rust: nightly
This will test all three release channels, but any breakage in nightly will not fail your overall build. Please see the Travis CI Rust documentation for more information.

GitLab CI
To test your package on GitLab CI, here is a sample .gitlab-ci.yml file:


stages:
  - build

rust-latest:
  stage: build
  image: rust:latest
  script:
    - cargo build --verbose
    - cargo test --verbose

rust-nightly:
  stage: build
  image: rustlang/rust:nightly
  script:
    - cargo build --verbose
    - cargo test --verbose
  allow_failure: true
This will test on the stable channel and nightly channel, but any breakage in nightly will not fail your overall build. Please see the GitLab CI for more information.

builds.sr.ht
To test your package on sr.ht, here is a sample .build.yml file. Be sure to change <your repo> and <your project> to the repo to clone and the directory where it was cloned.


image: archlinux
packages:
  - rustup
sources:
  - <your repo>
tasks:
  - setup: |
      rustup toolchain install nightly stable
      cd <your project>/
      rustup run stable cargo fetch
  - stable: |
      rustup default stable
      cd <your project>/
      cargo build --verbose
      cargo test --verbose
  - nightly: |
      rustup default nightly
      cd <your project>/
      cargo build --verbose ||:
      cargo test --verbose  ||:
  - docs: |
      cd <your project>/
      rustup run stable cargo doc --no-deps
      rustup run nightly cargo doc --no-deps ||:
This will test and build documentation on the stable channel and nightly channel, but any breakage in nightly will not fail your overall build. Please see the builds.sr.ht documentation for more information.

<h1> Cargo Home </h1>
The "Cargo home" functions as a download and source cache. When building a crate, Cargo stores downloaded build dependencies in the Cargo home. You can alter the location of the Cargo home by setting the CARGO_HOME environmental variable. The home crate provides an API for getting this location if you need this information inside your Rust crate. By default, the Cargo home is located in $HOME/.cargo/.

Please note that the internal structure of the Cargo home is not stabilized and may be subject to change at any time.

The Cargo home consists of following components:

Files:
config Cargo's global configuration file, see the config entry in the reference.

credentials Private login credentials from cargo login in order to log in to a registry.

.crates.toml This hidden file contains package information of crates installed via cargo install. Do NOT edit by hand!

Directories:
bin The bin directory contains executables of crates that were installed via cargo install or rustup. To be able to make these binaries accessible, add the path of the directory to your $PATH environment variable.

git Git sources are stored here:

git/db When a crate depends on a git repository, Cargo clones the repo as a bare repo into this directory and updates it if necessary.

git/checkouts If a git source is used, the required commit of the repo is checked out from the bare repo inside git/db into this directory. This provides the compiler with the actual files contained in the repo of the commit specified for that dependency. Multiple checkouts of different commits of the same repo are possible.

registry Packages and metadata of crate registries (such as crates.io) are located here.

registry/index The index is a bare git repository which contains the metadata (versions, dependencies etc) of all available crates of a registry.

registry/cache Downloaded dependencies are stored in the cache. The crates are compressed gzip archives named with a .crate extension.

registry/src If a downloaded .crate archive is required by a package, it is unpacked into registry/src folder where rustc will find the .rs files.

Caching the Cargo home in CI
To avoid redownloading all crate dependencies during continuous integration, you can cache the $CARGO_HOME directory. However, caching the entire directory is often inefficient as it will contain downloaded sources twice. If we depend on a crate such as serde 1.0.92 and cache the entire $CARGO_HOME we would actually cache the sources twice, the serde-1.0.92.crate inside registry/cache and the extracted .rs files of serde inside registry/src. That can unnecessarily slow down the build as downloading, extracting, recompressing and reuploading the cache to the CI servers can take some time.

It should be sufficient to only cache the following directories across builds:

bin/
registry/index/
registry/cache/
git/db/
Vendoring all dependencies of a project
See the cargo vendor subcommand.

Clearing the cache
In theory, you can always remove any part of the cache and Cargo will do its best to restore sources if a crate needs them either by reextracting an archive or checking out a bare repo or by simply redownloading the sources from the web.

Alternatively, the cargo-cache crate provides a simple CLI tool to only clear selected parts of the cache or show sizes of its components in your command-line.



Build cache
Cargo stores the output of a build into the "target" directory. By default, this is the directory named target in the root of your workspace. To change the location, you can set the CARGO_TARGET_DIR environment variable, the build.target-dir config value, or the --target-dir command-line flag.

The directory layout depends on whether or not you are using the --target flag to build for a specific platform. If --target is not specified, Cargo runs in a mode where it builds for the host architecture. The output goes into the root of the target directory, separated based on whether or not it is a release build:

Directory	Description
target/debug/	 Contains debug build output.
target/release/	 Contains release build output (with --release flag).
When building for another target with --target, the output is placed in a directory with the name of the target:

Directory	Example
target/<triple>/debug/	 target/thumbv7em-none-eabihf/debug/
target/<triple>/release/	 target/thumbv7em-none-eabihf/release/
Note: When not using --target, this has a consequence that Cargo will share your dependencies with build scripts and proc macros. RUSTFLAGS will be shared with every rustc invocation. With the --target flag, build scripts and proc macros are built separately (for the host architecture), and do not share RUSTFLAGS.

Within the profile directory (debug or release), artifacts are placed into the following directories:

Directory	Description
target/debug/	Contains the output of the package being built (the [[bin]] executables and [lib] library targets).
target/debug/examples/	 Contains examples ([[example]] targets).
Some commands place their output in dedicated directories in the top level of the target directory:

Directory	Description
target/doc/	Contains rustdoc documentation (cargo doc).
target/package/	Contains the output of the cargo package and cargo publish commands.
Cargo also creates several other directories and files needed for the build process. Their layout is considered internal to Cargo, and is subject to change. Some of these directories are:

Directory	Description
target/debug/deps/	 Dependencies and other artifacts.
target/debug/incremental/	 rustc incremental output, a cache used to speed up subsequent builds.
target/debug/build/	 Output from build scripts.
Dep-info files
Next to each compiled artifact is a file called a "dep info" file with a .d suffix. This file is a Makefile-like syntax that indicates all of the file dependencies required to rebuild the artifact. These are intended to be used with external build systems so that they can detect if Cargo needs to be re-executed. The paths in the file are absolute by default. See the build.dep-info-basedir config option to use relative paths.


# Example dep-info file found in target/debug/foo.d
/path/to/myproj/target/debug/foo: /path/to/myproj/src/lib.rs /path/to/myproj/src/main.rs
Shared cache
A third party tool, sccache, can be used to share built dependencies across different workspaces.

To setup sccache, install it with cargo install sccache and set RUSTC_WRAPPER environmental variable to sccache before invoking Cargo. If you use bash, it makes sense to add export RUSTC_WRAPPER=sccache to .bashrc. Alternatively, you can set [build.rustc-wrapper] in the Cargo configuration. Refer to sccache documentation for more details.