extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

const INCLUDED_TYPES: &[&str] = &[
    "spinlock_t",
    "mutex",
    "sk_buff",
    "net_device",
];
const INCLUDED_FUNCTIONS: &[&str] = &[
    "spin_lock",
    "printk",
    "krealloc",
    "kfree",
];
const INCLUDED_VARS: &[&str] = &["__this_module", "THIS_MODULE"];
const UNSUPPORTED_ARGS: &[&str] = &[
    "-mfunction-return=thunk-extern",
    "-fzero-call-used-regs=used-gpr",
    "-fconserve-stack",
    "-mrecord-mcount",
    "-ftrivial-auto-var-init=zero",
    "-Wimplicit-fallthrough=5",
    "-Wno-maybe-uninitialized",
    "-Wno-alloc-size-larger-than",
    "-mno-thumb-interwork",
    "-fno-caller-saves",
];

fn main() {
    let target = env::var("TARGET").unwrap();
    println!("Target={}", target);

    let mut builder = bindgen::Builder::default()
        .use_core()
        .detect_include_paths(false)
        .ctypes_prefix("c_types")
        .no_copy(".*")
        .derive_default(true)
        .derive_debug(false)
        .rustfmt_bindings(true)
        .opaque_type("xregs_state")
        .clang_arg(format!("--target={}", target));

    let output = String::from_utf8(
        Command::new("make")
            .arg("-C")
            .arg("kernel-cflags-finder")
            .arg("-s")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    Command::new("make")
        .arg("-C")
        .arg("kernel-cflags-finder")
        .arg("clean");

    println!("Output is: {}", output);

    let mut flags = "";
    for line in output.split("\n") {
        if line.contains("-nostdinc") {
            flags = line;
        }
    }

    Command::new("make")
        .arg("-C")
        .arg("kernel-cflags-finder")
        .arg("clean");

    println!("Output is {}", flags);

    let flags = flags.replace("\n", "");
    for arg in flags.split(" ") {
        if !UNSUPPORTED_ARGS.contains(&arg) {
            println!("Adding supported arg {}", arg);
            builder = builder.clang_arg(arg.to_string());
        } else {
            println!("Ignoring unsupported arg {}", arg);
        }
    }

    println!("cargo:rerun-if-changed=src/bindgen_helper.h");
    builder = builder.header("src/bindgen_helper.h");

    for t in INCLUDED_TYPES {
        builder = builder.allowlist_type(t);
    }
    for f in INCLUDED_FUNCTIONS {
        builder = builder.allowlist_function(f);
    }
    for v in INCLUDED_VARS {
        builder = builder.allowlist_var(v);
    }
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
