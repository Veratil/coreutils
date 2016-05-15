
use common::*;
use std;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ArgumentType {
    NoArgument,
    OptionalArgument,
    RequiredArgument,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Argument {
    pub match_args: Vec<&'static str>,
    pub arg_option: ArgumentType,
}

// This is for exploding possible combined single flags (e.g. -dRba)
//  into separate arguments: -d -R -b -a
fn preparse_args(args: Vec<String>, long_opts: &Vec<Argument>) -> Vec<String> {
    let mut newargs: Vec<String> = Vec::new();
    'mainloop: for arg in args {
        if arg.starts_with("-") && !arg.starts_with("--") {
            // we possibly have multiple single char options
            if arg.len() > 2 {
                let mut pos = 1;
                'chars: for c in arg[1..].to_string().chars() {
                    'longopts: for argu in long_opts {
                        // if it's a single char option and is a RequiredArgument, move along
                        if argu.match_args.contains(&c.to_string().as_str()) {
                            if argu.arg_option == ArgumentType::RequiredArgument {
                                newargs.push(String::from(format!("-{}", arg[pos..].to_string())));
                                continue 'mainloop;
                            }
                            else {
                                newargs.push(String::from(format!("-{}", c)));
                                break 'longopts;
                            }
                        }
                    } // 'longopts: for argu in long_opts
                    pos = pos + 1;
                } // 'chars: for c in arg[1..]
            } // if arg.len() > 2
        } // if arg.starts_with("-")
        newargs.push(arg.clone());
    }
    println!("{:?}", newargs);
    newargs
}

// -1 = help | version
//  0 = we're all good here
//  1 = error
// Vec = remaining arguments
pub fn parse_args(args: Vec<String>, long_opts: &Vec<Argument>, mut opts: &mut CpOptions) -> (i32, Option<Vec<String>>) { 
    let args = preparse_args(args, &long_opts); // replace args with exploded args
    let mut argp = 1; // Start at 1, 0 is basename
    loop {
        if argp >= args.len() { break; }
        let arg = &args[argp];
        if      arg == "--help" { print_help(); return (0, None); }
        else if arg == "--version" { print_version(); return (0, None); }
        else {
            let argarg: String;
            let mut argopt: Option<String>;
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
                        for argu in long_opts {
                            if argu.match_args.contains(&argarg.as_str()) && argu.arg_option == ArgumentType::RequiredArgument {
                                if argp+1 >= args.len() {
                                    print_missing_argument(argarg.as_str());
                                    print_cp_help();
                                    return (1, None);
                                }
                                argopt = Some(args[argp+1].to_string());
                                argp = argp + 1;
                                break;
                            }
                        }
                    }
                }
            }
            // Then try -a argument
            else if arg.starts_with("-") {
                argarg = arg[1..2].to_string();
                //println!("Found {}", argarg);
                argopt = None;
                for argu in long_opts {
                    if argu.match_args.contains(&argarg.as_str()) && argu.arg_option == ArgumentType::RequiredArgument {
                        //println!("Matched with argument: {}", argarg);
                        if arg[1..].to_string().len() > 1 {
                            // rest of argarg (e.g. -ttest, argarg = t && argopt = test)
                            argopt = Some(arg[2..].to_string());
                            //println!("argopt..={}", argopt.as_ref().unwrap());
                        }
                        else {
                            if argp+1 >= args.len() {
                                print_missing_argument(argarg.as_str());
                                print_cp_help();
                                return (1, None);
                            }
                            argopt = Some(args[argp+1].to_string());
                            //println!("argopt+1={}", argopt.as_ref().unwrap());
                            argp = argp + 1;
                        }
                        break;
                    }
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
fn parse_argument(arg: String, argopt: Option<String>, opts: &mut CpOptions) -> i32 {
    match arg.as_ref() {
        "a" | "archive" => {
            opts.dereference = DereferenceSymlink::Never;
            opts.preserve.links = true;
            opts.preserve.ownership = true;
            opts.preserve.mode = true;
            opts.preserve.timestamps = true;
            opts.require_preserve = true;
            // FIXME: Detect selinux
            opts.preserve.security_context = true;
            opts.preserve.xattr = true;
            opts.reduce_diagnostics = true;
            opts.recursive = true;
        }
        "attributes-only" => opts.data_copy_required = true,
        "b" | "backup" => {
            opts.make_backups = true;
            // Check if -n --no-clobber has been set, quit if so
            if opts.interactive == Interactive::AlwaysNo {
                println!("{0}: options --backup and --no-clobber are mutually exclusive",
                    std::env::args().nth(0).unwrap());
                return -2;
            }
            opts.backup_type = if argopt.is_some() {
                match argopt.as_ref().unwrap().as_str() {
                    // XXX: Is there a better way to do this?
                    "n"        => {
                        print_ambiguous_argument(argopt.unwrap().as_str(), "backup type", BACKUP_OPTIONS);
                        return -2;
                    }
                          "no" | "non" | "none"                                               => BackupType::NoBackups,
                    "o" | "of" | "off"                                                        => BackupType::NoBackups,
                          "nu" | "num" | "numb" | "numbe" | "number" | "numbere" | "numbered" => BackupType::Numbered,
                    "t"                                                                       => BackupType::Numbered,
                    "e" | "ex" | "exi" | "exis" | "exist" | "existi" | "existin" | "existing" => BackupType::NumberedExisting,
                          "ni" | "nil"                                                        => BackupType::NumberedExisting,
                    "s" | "si" | "sim" | "simp" | "simpl" | "simple"                          => BackupType::Simple,
                          "ne" | "nev" | "neve" | "never"                                     => BackupType::Simple,
                    _          => {
                        print_invalid_argument(argopt.unwrap().as_str(), "backup type", BACKUP_OPTIONS);
                        return -2;
                    }
                }
            } else { BackupType::NoBackups };
        }
        "copy-contents" => opts.copy_contents = true,
        "d" => {
            opts.preserve.links = true;
            opts.dereference = DereferenceSymlink::Never;
        }
        "f" | "force" => {
            opts.interactive = Interactive::AlwaysYes;
            opts.unlink_dest_after_failed_open = true;
        }
        "H" => opts.dereference = DereferenceSymlink::CommandLineArguments,
        "i" | "interactive" => opts.interactive = Interactive::AskUser,
        "l" | "link" => {
            if opts.symbolic_link {
                println!("{0}: cannot make both hard and symbolic links",
                    std::env::args().nth(0).unwrap());
                return -2;
            }
            opts.hard_link = true;
        }
        "L" | "dereference" => opts.dereference = DereferenceSymlink::Always,
        "n" | "no-clobber" => {
            // Check if -b --backup has been set, quit if so
            if opts.make_backups {
                println!("{0}: options --backup and --no-clobber are mutually exclusive",
                    std::env::args().nth(0).unwrap());
                return -2;
            }
            opts.interactive = Interactive::AlwaysNo;
        }
        "P" | "no-dereference" => opts.dereference = DereferenceSymlink::Never,
        "p" | "preserve" => {
            if argopt.is_some() {
                let unopt = argopt.unwrap(); // can't do argopt.unwrap().split for &str lifetime
                let plist: Vec<&str> = unopt.split(',').collect();
                for attr in plist {
                    match attr {
                        // XXX: Is there a better way to do this?
                        "m" | "mo" | "mod" | "mode" => {
                            opts.preserve.mode = true;
                            opts.explicit_no_preserve_mode = false;
                        }
                        "o" | "ow" | "own" | "owne" | "owner" | "owners" | "ownersh" | "ownershi" | "ownership"
                            => opts.preserve.ownership = true,
                        "t" | "ti" | "tim" | "time" | "times" | "timest" | "timesta" | "timestam" | "timestamp" | "timestamps"
                            => opts.preserve.timestamps = true,
                        "l" | "li" | "lin" | "link" | "links"
                            => opts.preserve.links = true,
                        "c" | "co" | "con" | "cont" | "conte" | "contex" | "context" => {
                            opts.require_preserve_context = true;
                            opts.preserve.security_context = true;
                        }
                        "x" | "xa" | "xat" | "xatt" | "xattr" => {
                            opts.preserve.xattr = true;
                            opts.require_preserve_xattr = true;
                        }
                        "a" | "al" | "all" => {
                            opts.preserve.mode = true;
                            opts.preserve.timestamps = true;
                            opts.preserve.ownership = true;
                            opts.preserve.links = true;
                            opts.explicit_no_preserve_mode = false;
                            // FIXME: Detect selinux
                            opts.preserve.security_context = true;
                            opts.preserve.xattr = true;
                        }
                        _            => {
                            print_invalid_argument(attr, "--preserve", PRESERVE_OPTIONS);
                            return -2;
                        }
                    } // match attr
                } // for attr in plist
            } // if argopt.is_some()
            else {
                opts.preserve.ownership = true;
                opts.preserve.mode = true;
                opts.preserve.timestamps = true;
            }
            opts.require_preserve = true;
        }
        "no-preserve" => {
            if argopt.is_none() { return -1; }
            else {
                let unopt = argopt.unwrap();
                let plist: Vec<&str> = unopt.split(',').collect();
                for attr in plist {
                    match attr {
                        // XXX: Is there a better way to do this?
                        "m" | "mo" | "mod" | "mode" => {
                            opts.preserve.mode = false;
                            opts.explicit_no_preserve_mode = true;
                        }
                        "o" | "ow" | "own" | "owne" | "owner" | "owners" | "ownersh" | "ownershi" | "ownership"
                            => opts.preserve.ownership = false,
                        "t" | "ti" | "tim" | "time" | "times" | "timest" | "timesta" | "timestam" | "timestamp" | "timestamps"
                            => opts.preserve.timestamps = false,
                        "l" | "li" | "lin" | "link" | "links"
                            => opts.preserve.links = false,
                        "c" | "co" | "con" | "cont" | "conte" | "contex" | "context" => {
                            opts.require_preserve_context = false;
                            opts.preserve.security_context = false;
                        }
                        "x" | "xa" | "xat" | "xatt" | "xattr" => {
                            opts.preserve.xattr = false;
                            opts.require_preserve_xattr = false;
                        }
                        "a" | "al" | "all" => {
                            opts.preserve.mode = false;
                            opts.preserve.timestamps = false;
                            opts.preserve.ownership = false;
                            opts.preserve.links = false;
                            opts.explicit_no_preserve_mode = true;
                            // FIXME: Detect selinux
                            opts.preserve.security_context = false;
                            opts.preserve.xattr = false;
                        }
                        _            => {
                            print_invalid_argument(attr, "--no-preserve", PRESERVE_OPTIONS);
                            return -2;
                        }
                    } // match attr
                } // for attr in plist
            }
        }
        "parents" => opts.parents_option = true,
        "R" | "r" | "recursive" => opts.recursive = true,
        "reflink" => {
            opts.reflink_mode = if argopt.is_some() {
                match argopt.as_ref().unwrap().as_str() {
                    // XXX: Is there a better way to do this?
                    "a" => {
                        print_ambiguous_argument(argopt.unwrap().as_str(), "--reflink", REFLINK_OPTIONS);
                        return -2;
                    }
                    "al" | "alw" | "alwa" | "alway" | "always" => ReflinkType::Always,
                    "au" | "aut" | "auto" => ReflinkType::Auto,
                    _ => {
                        print_invalid_argument(argopt.unwrap().as_str(), "--reflink", REFLINK_OPTIONS);
                        return -2;
                    }
                }
            } else { ReflinkType::Always };
        }
        "remove-destination" => opts.unlink_dest_before_opening = true,
        "sparse" => {
            if argopt.is_none() { panic!("--sparse requires when list"); }
            else {
                opts.sparse_mode = match argopt.as_ref().unwrap().as_str() {
                    // XXX: Is there a better way to do this?
                    "a" => {
                        print_ambiguous_argument(argopt.unwrap().as_str(), "--sparse", SPARSE_OPTIONS);
                        return -2;
                    }
                          "al" | "alw" | "alwa" | "alway" | "always" => SparseType::Always,
                          "au" | "aut" | "auto" => SparseType::Auto,
                    "n" | "ne" | "nev" | "neve" | "never" => SparseType::Never,
                    _ => {
                        print_invalid_argument(argopt.unwrap().as_str(), "--sparse", SPARSE_OPTIONS);
                        return -2;
                    }
                }
            }
        }
        "strip-trailing-slashes" => opts.remove_trailing_slashes = true,
        "s" | "symbolic-link" => {
            if opts.hard_link {
                println!("{0}: cannot make both hard and symbolic links",
                    std::env::args().nth(0).unwrap());
                return -2;
            }
            opts.symbolic_link = true;
        }
        "S" | "suffix" => {
            opts.make_backups = true;
            opts.backup_suffix_string = argopt.unwrap();
        }
        "t" | "target-directory" => {
            if opts.target_directory.len() > 0 {
                println!("{0}: multiple target directories specified",
                    std::env::args().nth(0).unwrap());
                return -3;
            }
            if opts.no_target_directory {
                println!("{0}: cannot combine --target-directory (-t) and --no-target-directory (-T)",
                    std::env::args().nth(0).unwrap());
                return -3;
            }
            // TODO: Test if target is real directory
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
        "x" | "one-file-system" => opts.one_file_system = true,
        "Z" | "context" => {
            // FIXME: Detect selinux
            if argopt.is_some() {
                opts.scontext = argopt.unwrap();
            }
            else {
                opts.set_security_context = true;
            }
        }
        
        _ => { return -1; }
    }
    0
} // parse_argument

