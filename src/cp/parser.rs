
use common::{BackupMethod, ReflinkWhen, SparseWhen, PreserveAttributes, Mode};
use common::{BACKUP_OPTIONS, PRESERVE_OPTIONS, REFLINK_OPTIONS, SPARSE_OPTIONS, VERSION};
use std;

// This is for exploding possible combined single flags (e.g. -dRba)
//  into separate arguments: -d -R -b -a
fn preparse_args(args: Vec<String>) -> Vec<String> {
    let mut newargs: Vec<String> = Vec::new();
    for arg in args {
        if arg.starts_with("-") && !arg.starts_with("--") {
            if arg.len() > 2 {
                let split_args: Vec<String> = arg[1..].to_string()
                                                        .chars()
                                                        .map(|s| format!("-{}",s).to_string())
                                                        .collect::<Vec<String>>();
                newargs.extend(split_args);
                continue;
            }
        }
        newargs.push(arg.clone());
    }
    newargs
}

// -1 = help | version
//  0 = we're all good here
//  1 = error
// Vec = remaining arguments
pub fn parse_args(args: Vec<String>, mut opts: &mut Mode) -> (i32, Option<Vec<String>>) { 
    let args = preparse_args(args); // replace args with exploded args
    let mut argp = 1; // Start at 1, 0 is basename
    loop {
        if argp >= args.len() { break; }
        let arg = &args[argp];
        if      arg == "--help" { print_help(); return (-1, None); }
        else if arg == "--version" { print_version(); return (-1, None); }
        else {
            let argarg: String;
            let argopt: Option<String>;
            // Try --abc arguments first
            if arg.starts_with("--") {
                match arg.find("=") {
                    Some(i) => {
                        argarg = arg[2..i].to_string();
                        argopt = Some(arg[i+1..].to_string());
                    }
                    None    => {
                        argarg = arg[2..].to_string();
                        argopt = None;
                    }
                }
            }
            // Then try -a argument
            else if arg.starts_with("-") {
                // Check for these two special options which take the next argument
                if arg == "-S" || arg == "-t" {
                    // One flag
                    argarg = arg[1..].to_string();
                    if argp+1 >= args.len() {
                        // special case error
                        print_missing_argument(argarg.as_str());
                        print_cp_help();
                        return (1, None);
                    }
                    argopt = Some(args[argp+1].clone());
                    argp = argp + 1; // skip over next arg
                }
                else {
                    argarg = arg[1..].to_string();
                    argopt = None;
                }
            }
            else {
                // No more options
                break;
            }
            let ret = parse_argument(argarg, argopt, &mut opts);

            match ret {
                // argument was good
                0 => { }
                // invalid option given
                -1 => {
                    println!("{0}: invalid option -- '{1}'", args[0], arg);
                    print_cp_help();
                    return (1, None);
                }
                // argument specific error
                -2 => {
                    print_cp_help();
                    return (1, None);
                }
                // argument specific error without --help suggest
                -3 => {
                    return (1, None);
                }
                // don't know
                _ => {
                    panic!("WHAT'D YOU DO?!");
                }
            }
        }
        argp = argp + 1;
    } // loop
    if argp >= args.len() {
        print_missing_files();
        print_cp_help();
        return (1, None);
    }
    (0, Some(args[argp..].to_vec()))
}

// arg = argument wihtout dash(es)
// argopt = if argument has options, will be Some(options), else None
// opts = Mode structure
fn parse_argument(arg: String, argopt: Option<String>, opts: &mut Mode) -> i32 {
    match arg.as_ref() {
        "a" | "archive" => opts.archive = true,
        "b" | "backup" => {
            // Check if -n --no-clobber has been set, quit if so
            if opts.no_clobber {
                println!("{0}: options --backup and --no-clobber are mutually exclusive",
                    std::env::args().nth(0).unwrap());
                return -2;
            }
            opts.backup = true;
            opts.backup_method = if argopt.is_some() {
                match argopt.as_ref().unwrap().as_str() {
                    // XXX: Is there a better way to do this?
                    "n"        => {
                        print_ambiguous_argument(argopt.unwrap().as_str(), "backup type", BACKUP_OPTIONS);
                        return -2;
                    }
                          "no" | "non" | "none"                                               => BackupMethod::none,
                    "o" | "of" | "off"                                                        => BackupMethod::off,
                          "nu" | "num" | "numb" | "numbe" | "number" | "numbere" | "numbered" => BackupMethod::numbered,
                    "t"                                                                       => BackupMethod::t,
                    "e" | "ex" | "exi" | "exis" | "exist" | "existi" | "existin" | "existing" => BackupMethod::existing,
                          "ni" | "nil"                                                        => BackupMethod::nil,
                    "s" | "si" | "sim" | "simp" | "simpl" | "simple"                          => BackupMethod::simple,
                          "ne" | "nev" | "neve" | "never"                                     => BackupMethod::never,
                    _          => {
                        print_invalid_argument(argopt.unwrap().as_str(), "backup type", BACKUP_OPTIONS);
                        return -2;
                    }
                }
            } else { BackupMethod::existing };
        }
        "copy-contents" => opts.copy_contents = true,
        "d" => opts.copy_as_symlinks = true,
        "f" | "force" => opts.force = true,
        "H" => opts.copy_real_file = true,
        "i" | "interactive" => opts.interactive = true,
        "l" | "link" => opts.copy_as_hardlink = true,
        "L" | "dereference" => opts.dereference = true,
        "n" | "no-clobber" => {
            // Check if -b --backup has been set, quit if so
            if opts.backup {
                println!("{0}: options --backup and --no-clobber are mutually exclusive",
                    std::env::args().nth(0).unwrap());
                return -2;
            }
            opts.no_clobber = true;
        }
        "P" | "no-dereference" => opts.no_dereference = true,
        "p" | "preserve" => {
            opts.preserve = true;
            if argopt.is_some() {
                let unopt = argopt.unwrap(); // can't do argopt.unwrap().split for &str lifetime
                let plist: Vec<&str> = unopt.split(',').collect();
                opts.preserve_attributes = PreserveAttributes::clean();
                for attr in plist {
                    match attr {
                        // XXX: Is there a better way to do this?
                        "m" | "mo" | "mod" | "mode"                                                                            => opts.preserve_attributes.mode = true,
                        "o" | "ow" | "own" | "owne" | "owner" | "owners" | "ownersh" | "ownershi" | "ownership"                => opts.preserve_attributes.ownership = true,
                        "t" | "ti" | "tim" | "time" | "times" | "timest" | "timesta" | "timestam" | "timestamp" | "timestamps" => opts.preserve_attributes.timestamps = true,
                        "l" | "li" | "lin" | "link" | "links"                                                                  => opts.preserve_attributes.links = true,
                        "c" | "co" | "con" | "cont" | "conte" | "contex" | "context"                                           => opts.preserve_attributes.context = true,
                        "x" | "xa" | "xat" | "xatt" | "xattr"                                                                  => opts.preserve_attributes.xattr = true,
                        "a" | "al" | "all"                                                                                     => opts.preserve_attributes.all = true,
                        _            => {
                            print_invalid_argument(attr, "--preserve", PRESERVE_OPTIONS);
                            return -2;
                        }
                    } // match attr
                } // for attr in plist
            } // if argopt.is_some()
        }
        "no-preserve" => {
            opts.no_preserve = true;
            if argopt.is_none() { return -1; }
            else {
                let unopt = argopt.unwrap();
                let plist: Vec<&str> = unopt.split(',').collect();
                opts.nopreserve_attributes = PreserveAttributes::clean();
                for attr in plist {
                    match attr {
                        // XXX: Is there a better way to do this?
                        "m" | "mo" | "mod" | "mode"                                                                            => opts.preserve_attributes.mode = true,
                        "o" | "ow" | "own" | "owne" | "owner" | "owners" | "ownersh" | "ownershi" | "ownership"                => opts.preserve_attributes.ownership = true,
                        "t" | "ti" | "tim" | "time" | "times" | "timest" | "timesta" | "timestam" | "timestamp" | "timestamps" => opts.preserve_attributes.timestamps = true,
                        "l" | "li" | "lin" | "link" | "links"                                                                  => opts.preserve_attributes.links = true,
                        "c" | "co" | "con" | "cont" | "conte" | "contex" | "context"                                           => opts.preserve_attributes.context = true,
                        "x" | "xa" | "xat" | "xatt" | "xattr"                                                                  => opts.preserve_attributes.xattr = true,
                        "a" | "al" | "all"                                                                                     => opts.preserve_attributes.all = true,
                        _            => {
                            print_invalid_argument(attr, "--no-preserve", PRESERVE_OPTIONS);
                            return -2;
                        }
                    } // match attr
                } // for attr in plist
            }
        }
        "parents" => opts.parents = true,
        "R" | "r" | "recursive" => opts.recursive = true,
        "reflink" => {
            opts.reflink = true;
            opts.reflink_when = if argopt.is_some() {
                match argopt.as_ref().unwrap().as_str() {
                    // XXX: Is there a better way to do this?
                    "a" => {
                        print_ambiguous_argument(argopt.unwrap().as_str(), "--reflink", REFLINK_OPTIONS);
                        return -2;
                    }
                    "al" | "alw" | "alwa" | "alway" | "always" => ReflinkWhen::always,
                    "au" | "aut" | "auto" => ReflinkWhen::auto,
                    _ => {
                        print_invalid_argument(argopt.unwrap().as_str(), "--reflink", REFLINK_OPTIONS);
                        return -2;
                    }
                }
            } else { ReflinkWhen::always };
        }
        "remove-destination" => opts.remove_destination = true,
        "sparse" => {
            opts.sparse = true;
            if argopt.is_none() { panic!("--sparse requires when list"); }
            else {
                opts.sparse_when = match argopt.as_ref().unwrap().as_str() {
                    // XXX: Is there a better way to do this?
                    "a" => {
                        print_ambiguous_argument(argopt.unwrap().as_str(), "--sparse", SPARSE_OPTIONS);
                        return -2;
                    }
                          "al" | "alw" | "alwa" | "alway" | "always" => SparseWhen::always,
                          "au" | "aut" | "auto" => SparseWhen::auto,
                    "n" | "ne" | "nev" | "neve" | "never" => SparseWhen::never,
                    _ => {
                        print_invalid_argument(argopt.unwrap().as_str(), "--sparse", SPARSE_OPTIONS);
                        return -2;
                    }
                }
            }
        }
        "strip-trailing-slashes" => opts.strip_trailing_slashes = true,
        "s" | "symbolic-link" => opts.symbolic_link = true,
        "S" | "suffix" => opts.suffix = argopt.unwrap(),
        "t" | "target-directory" => {
            if opts.no_target_directory {
                println!("{0}: cannot combine --target-directory (-t) and --no-target-directory (-T)",
                    std::env::args().nth(0).unwrap());
                return -3;
            }
            opts.target_directory = argopt.unwrap();
        }
        "T" | "no-target-directory" => {
            if opts.target_directory.len() > 0 {
                println!("{0}: cannot combine --target-directory (-t) and --no-target-directory (-T)",
                    std::env::args().nth(0).unwrap());
                return -3;
            }
            opts.no_target_directory = true;
        }
        "u" | "update" => opts.update = true,
        "v" | "verbose" => opts.verbose = true,
        "x" | "one-file-system" => opts.skip_subdirectories = true,
        "Z" => opts.selinux_context = true,
        "context" => {
            if argopt.is_some() { opts.context = argopt.unwrap(); }
            else { panic!("No selinux context specified"); }
        }
        
        _ => { return -1; }
    }
    0
} // parse_argument

fn print_missing_files() {
    println!("{0}: missing file operand",
        std::env::args().nth(0).unwrap());
}

fn print_missing_argument(forarg: &str) {
    println!("{0}: option requires an argument -- '{1}'",
        std::env::args().nth(0).unwrap(), forarg);
}

fn print_bad_argument(badt: &str, arg: &str, forarg: &str, explain: &str) {
    println!("{0}: {1} argument '{2}' for '{3}'\nValid arguments are:\n{4}",
        std::env::args().nth(0).unwrap(), badt, arg, forarg, explain);
}

fn print_ambiguous_argument(ambarg: &str, forarg: &str, explain: &str) {
    print_bad_argument("ambiguous", ambarg, forarg, explain);
}

fn print_invalid_argument(invarg: &str, forarg: &str, explain: &str) {
    print_bad_argument("invalid", invarg, forarg, explain);
}

fn print_cp_help() {
    println!("Try '{0} --help' for more information.", std::env::args().nth(0).unwrap());
}

fn print_version() {
    println!("{} {}", std::env::args().nth(0).unwrap(), VERSION);
}

fn print_help() {
    let msg = format!(
r#"Usage: {0} [OPTION]... [-T] SOURCE DEST
  or:  {0} [OPTION]... SOURCE... DIRECTORY
  or:  {0} [OPTION]... -t DIRECTORY SOURCE...
Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.

Mandatory arguments to long options are mandatory for short options too.
TODO: WRITE ARGUMENT TEXT"#,
        std::env::args().nth(0).unwrap());
    println!("{}", msg);
}
