// -*- coding: utf-8 -*-
// vi: set sts=4 ts=4 sw=4 et ft=rust:

//! pentry command is example of pentry library.
//!

extern crate getopts;
extern crate libc;
extern crate pentry;

fn usage(prog: &str, opts: getopts::Options) {
    let brief = format!(r#"Show some process information.

USAGE: {0} -h|--help
       {0} [-P|--parent] [PID [PID...]]"#,
                        prog);

    print!("{}", opts.usage(&brief));
}

fn show(entry: &pentry::Process, indent: &str) {
    if let Some(path) = entry.path() {
        println!("{:5}\t{:5}\t{}{}", entry.pid(), entry.ppid(), indent, path);
    } else {
        println!("{:5}\t{:5}\t{}{}",
                 entry.pid(),
                 entry.ppid(),
                 indent,
                 entry.name());
    }
}

fn head() {
    println!("  PID\t PPID\tPATH");
}

fn inspect(pid: i32, parent: bool) {
    let entry = pentry::find(pid).unwrap();
    if parent {
        let entry2 = pentry::find(entry.ppid()).unwrap();
        show(&entry2, "");
        show(&entry, "┗ ");
    } else {
        show(&entry, "");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    // Setup options.
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "Show this usage message.");
    opts.optflag("P", "parent", "Inspect parent process entry.");

    // Parse args.
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Check flags.
    if matches.opt_present("h") {
        usage(&program, opts);
        return;
    }
    let parent: bool = matches.opt_present("P");

    // Check Params.
    if matches.free.is_empty() {
        head();
        let pid: i32;
        unsafe {
            pid = libc::getpid() as i32;
        }
        inspect(pid, parent);
    } else {
        head();
        for arg in matches.free {
            let pid = arg.parse::<i32>();
            inspect(pid.unwrap(), parent);
        }
    }
}
