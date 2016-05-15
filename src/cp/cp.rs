#![crate_name = "uu_cp"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Kevin Zander <veratil@gmail.com>
 * (c) Jordy Dickinson <jordy.dickinson@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE file
 * that was distributed with this source code.
 */

#[macro_use]
extern crate uucore;

/*
  getopts was decided not to be used since it could not handle the 3 option
    recursive flag: -r -R --recursive
  argparse I can't use since it doesn't support --cmd=something support
  Rolling my own parser isn't too hard, just the sheer size of options
 */

use common::*;
use parser::*;

mod parser;
mod common;

//use std::fs;
//use std::io::{ErrorKind, Result, Write};
//use std::path::Path;
//use uucore::fs::{canonicalize, CanonicalizeMode};

pub fn uumain(args: Vec<String>) -> i32 {
    let mut opts: CpOptions = CpOptions::new();

    let long_opts: Vec<Argument> = vec![
        Argument { match_args: vec!["a", "archive"],             arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["attribute-only"],           arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["b", "backup"],              arg_option: ArgumentType::OptionalArgument },
        Argument { match_args: vec!["copy-contents"],            arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["L", "dereference"],         arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["f", "force"],               arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["i", "interactive"],         arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["l", "link"],                arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["n", "no-clobber"],          arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["P", "no-dereference"],      arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["no-preserve"],              arg_option: ArgumentType::RequiredArgument },
        Argument { match_args: vec!["T", "no-target-directory"], arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["x", "one-file-system"],     arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["parents"],                  arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["p"],                        arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["preserve"],                 arg_option: ArgumentType::OptionalArgument },
        Argument { match_args: vec!["R", "r", "recursive"],      arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["remove-destination"],       arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["sparse"],                   arg_option: ArgumentType::RequiredArgument },
        Argument { match_args: vec!["reflink"],                  arg_option: ArgumentType::OptionalArgument },
        Argument { match_args: vec!["strip-trailing-slashes"],   arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["S", "suffix"],              arg_option: ArgumentType::RequiredArgument },
        Argument { match_args: vec!["s", "symbolic-link"],       arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["t", "target-directory"],    arg_option: ArgumentType::RequiredArgument },
        Argument { match_args: vec!["u", "update"],              arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["v", "verbose"],             arg_option: ArgumentType::NoArgument },
        Argument { match_args: vec!["Z", "context"],             arg_option: ArgumentType::OptionalArgument },
        //Argument { match_args: vec![""], arg_option: ArgumentType::NoArgument },
    ];

    let (ret, sdargs) = parser::parse_args(args, &long_opts, &mut opts);

    if ret != 0 || sdargs.is_none() {
        println!("{:?}", ret);
        return ret;
    }

    // Post-parse CpOptions for conflicts and option combinations
    if opts.reflink_mode == ReflinkType::Always && opts.sparse_mode != SparseType::Auto {
        println!("{0}: --reflink can be used only with --sparse=auto",
            std::env::args().nth(0).unwrap());
        print_cp_help();
        return 1;
    }
    if opts.dereference == DereferenceSymlink::Undefined {
        if opts.recursive && !opts.hard_link {
            opts.dereference = DereferenceSymlink::Never;
        }
        else {
            opts.dereference = DereferenceSymlink::Always;
        }
    }
    if opts.recursive {
        opts.copy_as_regular = true;
    }
    if opts.unlink_dest_after_failed_open && (opts.hard_link || opts.symbolic_link) {
        opts.unlink_dest_before_opening = true;
    }
    if (opts.set_security_context || opts.scontext.len() > 0) && !opts.require_preserve_context {
        opts.preserve.security_context = false;
    }
    if opts.preserve.security_context && (opts.set_security_context || opts.scontext.len() > 0) {
        println!("{0}: cannot set target context and preserve it",
            std::env::args().nth(0).unwrap());
        print_cp_help();
        return 1;
    }
    // FIXME: Detect selinux
    /*if opts.require_preserve_context && !selinux_enabled {
        println!("{0}: cannot preserve security context without an SELinux-enabled kernel",
            std::env::args().nth(0).unwrap());
    }*/
    /*if opts.scontext.len() > 0 && setfscreatecon(se_const(opts.scontext)) < 0 {
        println!("{0}: failed to set default file creation context to {1}",
            std::env::args().nth(0).unwrap(), opts.scontext);
        print_cp_help();
        return 1;
    }*/

    println!("{:?}", opts);
    println!("{:?}", ret);
    println!("{:?}", sdargs);
    println!("");

    copy(sdargs.unwrap(), &opts)
} // uumain()

fn copy(files: Vec<String>, opts: &CpOptions) -> i32 {
    if files.len() < 2 {
        print_missing_destination_file(&files[0]);
        print_cp_help();
        return 1;
    }
    /*
    Usage: cp [OPTION]... [-T] SOURCE DEST
      or:  cp [OPTION]... SOURCE... DIRECTORY
      or:  cp [OPTION]... -t DIRECTORY SOURCE...
    Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.
    */
    // (3rd usage) do we have a -t DIRECTORY defined?
    if opts.target_directory.len() > 0 {
        println!("Copying {:?} to directory {}", files, opts.target_directory);
    }
    // (1st usage) two names only
    else if files.len() == 2 { 
        println!("Copy {} to {}", files[0], files[1]);
    }
    // (2nd usage) multiple sources to one directory
    else {
        // remember slices are [inclusive..exclusive)
        println!("Copying {:?} to directory {}", files[0..files.len()-1].to_vec(), files[files.len()-1]);
    }
    // just return 1 for now
    1
}
/*
fn copy(matches: getopts::Matches) {
    let sources: Vec<String> = if matches.free.is_empty() {
        show_error!("Missing SOURCE argument. Try --help.");
        panic!()
    } else {
        // All but the last argument:
        matches.free[..matches.free.len() - 1].iter().cloned().collect()
    };
    let dest = if matches.free.len() < 2 {
        show_error!("Missing DEST argument. Try --help.");
        panic!()
    } else {
        // Only the last argument:
        Path::new(&matches.free[matches.free.len() - 1])
    };

    assert!(sources.len() >= 1);

    if sources.len() == 1 {
        let source = Path::new(&sources[0]);
        let same_file = paths_refer_to_same_file(source, dest).unwrap_or_else(|err| {
            match err.kind() {
                ErrorKind::NotFound => false,
                _ => {
                    show_error!("{}", err);
                    panic!()
                }
            }
        });

        if same_file {
            show_error!("\"{}\" and \"{}\" are the same file",
                source.display(),
                dest.display());
            panic!();
        }

        if let Err(err) = fs::copy(source, dest) {
            show_error!("{}", err);
            panic!();
        }
    } else {
        if !dest.is_dir() {
            show_error!("TARGET must be a directory");
            panic!();
        }

        for src in &sources {
            let source = Path::new(&src);

            if !source.is_file() {
                show_error!("\"{}\" is not a file", source.display());
                continue;
            }

            let mut full_dest = dest.to_path_buf();

            full_dest.push(source.to_str().unwrap());

            println!("{}", full_dest.display());

            let io_result = fs::copy(source, full_dest);

            if let Err(err) = io_result {
                show_error!("{}", err);
                panic!()
            }
        }
    }
}

pub fn paths_refer_to_same_file(p1: &Path, p2: &Path) -> Result<bool> {
    // We have to take symlinks and relative paths into account.
    let pathbuf1 = try!(canonicalize(p1, CanonicalizeMode::Normal));
    let pathbuf2 = try!(canonicalize(p2, CanonicalizeMode::Normal));

    Ok(pathbuf1 == pathbuf2)
}
*/
