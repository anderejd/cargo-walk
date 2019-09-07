#![forbid(unsafe_code)]
//#![forbid(warnings)]

extern crate cargo;
extern crate structopt;

use cargo::core::package::PackageSet;
use cargo::core::registry::PackageRegistry;
use cargo::core::resolver::Method;
use cargo::core::shell::Shell;
use cargo::core::Package;
use cargo::core::PackageId;
use cargo::core::PackageIdSpec;
use cargo::core::Resolve;
use cargo::core::Workspace;
use cargo::ops;
use cargo::util::important_paths;
use cargo::util::CargoResult;
use cargo::CliResult;
use cargo::Config;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

// COPY-PASTED from cargo-geiger, review this later. Is it needed for all cargo plugins?
#[derive(StructOpt)]
#[structopt(bin_name = "cargo")]
pub enum Opts {
    #[structopt(name = "walk")]
    /// Run a command for each level of a Rust crate dependency tree.
    Walk(Args),
}

#[derive(StructOpt)]
pub struct Args {
    #[structopt(
        long = "manifest-path",
        value_name = "PATH",
        parse(from_os_str)
    )]
    /// Path to Cargo.toml
    pub manifest_path: Option<PathBuf>,

    #[structopt(long = "verbose", short = "v", parse(from_occurrences))]
    /// Use verbose output (-vv very verbose/build.rs output)
    pub verbose: u32,

    #[structopt(long = "quiet", short = "q")]
    /// No output printed to stdout other than the tree
    pub quiet: Option<bool>,

    #[structopt(long = "color", value_name = "WHEN")]
    /// Coloring: auto, always, never
    pub color: Option<String>,

    #[structopt(long = "frozen")]
    /// Require Cargo.lock and cache are up to date
    pub frozen: bool,

    #[structopt(long = "locked")]
    /// Require Cargo.lock is up to date
    pub locked: bool,

    #[structopt(long = "offline")]
    /// Run without accessing the network
    pub offline: bool,

    subprocess_command: Vec<String>,
}

// COPY-PASTE from cargo-geiger.
// TODO: Review this later.
pub fn resolve<'a, 'cfg>(
    package_id: PackageId,
    registry: &mut PackageRegistry<'cfg>,
    ws: &'a Workspace<'cfg>,
    features: Option<String>,
    all_features: bool,
    no_default_features: bool,
) -> CargoResult<(PackageSet<'a>, Resolve)> {
    let features = std::rc::Rc::new(Method::split_features(
        &features.into_iter().collect::<Vec<_>>(),
    ));
    let method = Method::Required {
        dev_deps: true,
        features,
        all_features,
        uses_default_features: !no_default_features,
    };
    let prev = ops::load_pkg_lockfile(ws)?;
    let resolve = ops::resolve_with_previous(
        registry,
        ws,
        method,
        prev.as_ref(),
        None,
        &[PackageIdSpec::from_package_id(package_id)],
        true,
    )?;
    let packages = ops::get_resolved_packages(
        &resolve,
        PackageRegistry::new(ws.config())?,
    )?;
    Ok((packages, resolve))
}

// COPY-PASTE from cargo-geiger.
// TODO: Review this later.
pub fn get_workspace(
    config: &Config,
    manifest_path: Option<PathBuf>,
) -> CargoResult<Workspace> {
    let root = match manifest_path {
        Some(path) => path,
        None => important_paths::find_root_manifest_for_wd(config.cwd())?,
    };
    Workspace::new(&root, config)
}

// COPY-PASTE from cargo-geiger.
// TODO: Review this later.
pub fn get_registry<'a>(
    config: &'a Config,
    package: &Package,
) -> CargoResult<PackageRegistry<'a>> {
    let mut registry = PackageRegistry::new(config)?;
    registry.add_sources(Some(package.package_id().source_id()))?;
    Ok(registry)
}

fn real_main(args: &Args, config: &mut Config) -> CliResult {
    let target_dir = None;
    let unstable_flags = Vec::new();
    config.configure(
        args.verbose,
        args.quiet,
        &args.color,
        args.frozen,
        args.locked,
        args.offline,
        &target_dir,
        &unstable_flags,
    )?;
    let ws = get_workspace(config, args.manifest_path.clone())?;
    let root_package = ws.current()?;
    let mut registry = get_registry(config, &root_package)?;
    let all_features = true;
    let specific_features = None;
    let no_default_features = false;
    let (packages, _) = resolve(
        root_package.package_id(),
        &mut registry,
        &ws,
        specific_features,
        all_features,
        no_default_features,
    )?;
    let mut ids = packages.package_ids().collect::<Vec<_>>();
    let packages = registry.get(&ids)?; // TODO: Review why/if this is needed.
    ids.sort();
    let packs = packages.get_many(ids).unwrap();
    if args.subprocess_command.len() < 1 {
        panic!("Inner command missing.");
    }
    let (left, right) = args.subprocess_command.split_at(1);
    let cmd = &left[0];
    for p in packs {
        let _status = Command::new(cmd)
            .args(right)
            .arg(p.root())
            .status()
            .expect("failed to execute process");
    }
    Ok(())
}

fn main() {
    let mut config = match Config::default() {
        Ok(cfg) => cfg,
        Err(e) => {
            let mut shell = Shell::new();
            cargo::exit_with_error(e.into(), &mut shell)
        }
    };
    let Opts::Walk(args) = Opts::from_args();
    if let Err(e) = real_main(&args, &mut config) {
        let mut shell = Shell::new();
        cargo::exit_with_error(e, &mut shell)
    }
}
