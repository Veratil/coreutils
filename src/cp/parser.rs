
use common::{BackupMethod, ReflinkWhen, SparseWhen, PreserveAttributes, Mode};
use common::{BACKUP_OPTIONS, PRESERVE_OPTIONS, VERSION};
use std;

fn preparse_args(args: Vec<String>) -> Vec<String> {
    //let mut argp = 1;
    println!("args: {:?}", args);
    let mut nextargrequired: bool = true; // set to true for args[0]
    let mut newargs: Vec<String> = Vec::new();
    //newargs.push(args[0].to_string());
    for arg in args {
        //println!("newargs: {:?}", newargs);
        //if argp >= args.len() { break; }
        //let arg = &args[argp];
        println!("arg: {:?}", arg);
        if nextargrequired {
            println!("NAR");
            newargs.push(arg.clone());
            nextargrequired = false;
            continue;
        }
        else if arg.starts_with("--") {
            println!("SW--");
            newargs.push(arg.clone());
            continue;
        }
        else if arg.starts_with("-") {
            println!("SW-");
            if arg.len() > 2 {
                println!("SPL");
                let split_args: Vec<String> = arg[1..].to_string()
                                                        .chars()
                                                        .map(|s| format!("-{}",s).to_string())
                                                        .collect::<Vec<String>>();
                //argp = argp + split_args.len();
                newargs.extend(split_args);
                continue;
            }
            // Capture required argument
            else if arg == "-S" || arg == "-t" {
                println!("CRA");
                newargs.push(arg.clone());
                nextargrequired = true;
                //newargs.push(args[argp+1].clone());
                continue;
            }
        }
        println!("PSH");
        newargs.push(arg.clone());
        //argp = argp + 1;
    }
    println!("newargs: {:?}", newargs);
    newargs
}

// -1 = help | version
//  0 = we're all good here
//  1 = error
pub fn parse_args(args: Vec<String>, mut opts: &mut Mode) -> i32 { 
    println!("parse_args>preparse");
    let args = preparse_args(args);
    println!("parse_args>done");
    let mut argp = 1; // Start at 1, 0 is basename
    loop {
        if argp >= args.len() { break; }
        println!("looking at arg: {:?}", &args[argp]);
        let arg = &args[argp];
        //println!("arg: {}", arg);
        if      arg == "--help" { print_help(); return -1; }
        else if arg == "--version" { print_version(); return -1; }
        else {
            let argarg: String;
            let argopt: Option<String>;
            // Try --abc arguments first
            if arg.starts_with("--") {
                match arg.find("=") {
                    Some(i) => {
                        argarg = arg[2..i].to_string();
                        argopt = Some(arg[i+1..].to_string());
                    } //ret = parse_arguments(arg[2..i].to_string(), Some(arg[i+1..].to_string()), &mut opts); }
                    None    => {
                        argarg = arg[2..].to_string();
                        argopt = None;
                    } //ret = parse_arguments(arg[2..].to_string(), None, &mut opts); }
                }
            }
            else if arg.starts_with("-") {
                if arg == "-S" || arg == "-t" {
                    // One flag
                    argarg = arg[1..].to_string();
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
                    return 1;
                }
                // argument specific error
                -2 => {
                    print_cp_help();
                    return 1;
                }
                // don't know
                _ => {
                    panic!("WHAT'D YOU DO?!");
                }
            }
        }
        argp = argp + 1;
    } // loop
    0
}

// arg = argument wihtout dash(es)
// argopt = if argument has options, will be Some(options), else None
// opts = Mode structure
fn parse_argument(arg: String, argopt: Option<String>, opts: &mut Mode) -> i32 {
    match arg.as_ref() {
        "a" | "archive" => opts.archive = true,
        // TODO: mutually exclusive with -n --no-clobber
        // cp: options --backup and --no-clobber are mutually exclusive
        // Try 'cp --help' for more information.
        "b" | "backup" => {
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
        // TODO: mutually exclusive with -b --backup
        // cp: options --backup and --no-clobber are mutually exclusive
        // Try 'cp --help' for more information.
        "n" | "no-clobber" => opts.no_clobber = true,
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
                match argopt.unwrap().as_ref() {
                    // FIXME: do smart? matching 'al' == 'always', 'a' == ambiguous
                    "always" => ReflinkWhen::always,
                    "auto" => ReflinkWhen::auto,
                    _ => { panic!("Not implemented error function"); }
                }
            } else { ReflinkWhen::always };
        }
        "remove-destination" => opts.remove_destination = true,
        "sparse" => {
            opts.sparse = true;
            if argopt.is_none() { panic!("--sparse requires when list"); }
            else {
                opts.sparse_when = match argopt.unwrap().as_ref() {
                    // FIXME: do smart? matching 'al' == 'always', 'a' == ambiguous
                    "always" => SparseWhen::always,
                    "auto" => SparseWhen::auto,
                    "never" => SparseWhen::never,
                    _ => { panic!("Unknown sparse when"); }
                }
            }
        }
        "strip-trailing-slashes" => opts.strip_trailing_slashes = true,
        "s" | "symbolic-link" => opts.symbolic_link = true,
        "S" | "suffix" => opts.suffix = argopt.unwrap(),
        // TODO: -t and -T are mutually exclusive
        // cp: cannot combine --target-directory (-t) and --no-target-directory (-T)
        "t" | "target-directory" => opts.target_directory = argopt.unwrap(),
        "T" | "no-target-directory" => opts.no_target_directory = true,
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
    /*
    if      arg == "-a" || arg == "--archive" { opts.archive = true; }
    else if arg == "-b" { opts.backup = true; }
    else if arg.starts_with("--backup") {
        opts.backup = true;
        if arg.len() > "--backup".len() {
            let btype = &arg["--backup=".len()..];
            match btype {
                // FIXME: cp uses smart? match, 'n' == ambiguous argument, while 'no' == 'none'
                "none"     => { opts.backup_method = BackupMethod::none; }
                "off"      => { opts.backup_method = BackupMethod::off; }
                "numbered" => { opts.backup_method = BackupMethod::numbered; }
                "t"        => { opts.backup_method = BackupMethod::t; }
                "existing" => { opts.backup_method = BackupMethod::existing; }
                "nil"      => { opts.backup_method = BackupMethod::nil; }
                "simple"   => { opts.backup_method = BackupMethod::simple; }
                "never"    => { opts.backup_method = BackupMethod::never; }
                _          => {
                    print_invalid_argument(btype, "backup type", BACKUP_OPTIONS);
                    return -2;
                }
            }
        }
    }
    else if arg == "--copy-contents" { opts.copy_contents = true; }
    else if arg == "-d" { opts.copy_as_symlinks = true; }
    // TODO: Make sure does not cancel -i --interactive
    // TODO: Must ignore when -n --no-clobber is used
    else if arg == "-f" || arg == "--force" { opts.force = true; }
    else if arg == "-H" { opts.copy_real_file = true; }
    // TODO: Make override previous -n option
    else if arg == "-i" || arg == "--interactive" { opts.interactive = true; }
    else if arg == "-l" || arg == "--link" { opts.copy_as_hardlink = true; }
    else if arg == "-L" || arg == "--dereference" { opts.dereference = true; }
    // TODO: Make override previous -i option
    // TODO: Ensure mutual exclusion with -b --backup option
    else if arg == "-n" || arg == "--no-clobber" { opts.no_clobber = true; }
    else if arg == "-P" || arg == "--no-dereference" { opts.no_dereference = true; }
    else if arg == "-p" || arg.starts_with("--preserve") {
        opts.preserve = true;
        if arg.len() > "--preserve".len() {
            let plist: Vec<&str> = arg["--preserve=".len()..].split(',').collect();
            opts.preserve_attributes = PreserveAttributes::clean();
            for attr in plist {
                match attr {
                    // FIXME: cp uses smart? match, 'm' == 'mode', 'a' == 'all', etc.
                    "mode"       => { opts.preserve_attributes.mode = true; }
                    "ownership"  => { opts.preserve_attributes.ownership = true; }
                    "timestamps" => { opts.preserve_attributes.timestamps = true; }
                    "links"      => { opts.preserve_attributes.links = true; }
                    "context"    => { opts.preserve_attributes.context = true; }
                    "xattr"      => { opts.preserve_attributes.xattr = true; }
                    "all"        => { opts.preserve_attributes.all = true; }
                    _            => {
                        print_invalid_argument(attr, "--preserve", PRESERVE_OPTIONS);
                        return -2;
                    }
                }
            }
        }
    }
    else if arg.starts_with("--no-preserve") {
        opts.no_preserve = true;
        if arg.len() < "--no-preserve".len() {
            // FIXME: Replace with proper error handling
            panic!("--no-preserve requires attribute list");
        }
        else {
            let plist: Vec<&str> = arg["--no-preserve=".len()..].split(',').collect();
            for attr in plist {
                match attr {
                    // FIXME: cp uses smart? match, 'm' == 'mode', 'a' == 'all', etc.
                    "mode"       => { opts.nopreserve_attributes.mode = true; }
                    "ownership"  => { opts.nopreserve_attributes.ownership = true; }
                    "timestamps" => { opts.nopreserve_attributes.timestamps = true; }
                    "links"      => { opts.nopreserve_attributes.links = true; }
                    "context"    => { opts.nopreserve_attributes.context = true; }
                    "xattr"      => { opts.nopreserve_attributes.xattr = true; }
                    "all"        => { opts.nopreserve_attributes.all = true; }
                    _            => {
                        print_invalid_argument(attr, "--no-preserve", PRESERVE_OPTIONS);
                        return -2;
                    }
                }
            }
        }
    }
    else if arg == "--parents" { opts.parents = true; }
    else if arg == "-R" || arg == "-r" || arg == "--recursive" { opts.recursive = true; }
    // TODO: Make overridden by --link, --symbolic-link, and --attributes-only
    else if arg.starts_with("--reflink") {
        opts.reflink = true;
        if arg.len() > "--reflink".len() {
            match &arg["--reflink=".len()..] {
                "always" => { opts.reflink_when = ReflinkWhen::always; }
                "auto"   => { opts.reflink_when = ReflinkWhen::auto; }
                // FIXME: Replace with proper error handling
                _        => { panic!("Unknown reflink when"); }
            }
        }
    }
    else if arg == "--remove-destination" { opts.remove_destination = true; }
    else if arg.starts_with("--sparse") {
        opts.sparse = true;
        if arg.len() < "--sparse".len() {
            // FIXME: Replace with proper error handling
            panic!("--sprase requires when list");
        }
        else {
            match &arg["--sparse=".len()..] {
                "always" => { opts.sparse_when = SparseWhen::always; }
                "auto"   => { opts.sparse_when = SparseWhen::auto; }
                "never"  => { opts.sparse_when = SparseWhen::never; }
                // FIXME: Replace with proper error handling
                _        => { panic!("Unknown sparse when"); }
            }
        }
    }
    else if arg == "--strip-trailing-slashes" { opts.strip_trailing_slashes = true; }
    else if arg == "-s" || arg == "--symbolic-link" { opts.symbolic_link = true; }
    else if arg == "-S" { panic!("-S is not implemented yet"); /* TODO: Add a way to get the next arg */ }
    else if arg.starts_with("--suffix") {
        if arg.len() < "--suffix=".len() {
            // FIXME: Replace with proper error handling
            panic!("No suffix defined");
        }
        else {
            opts.suffix = arg["--suffix=".len()..].to_string();
        }
    }
    else if arg == "-t" { panic!("-t is not implemented yet"); /* TODO: Add a way to get the next arg */ }
    else if arg.starts_with("--target-directory") {
        if arg.len() < "--target-directory=".len() {
            panic!("No target directory defined");
        }
        else {
            opts.target_directory = arg["--target-directory=".len()..].to_string();
        }
    }
    else if arg == "-u" || arg == "--update" { opts.update = true; }
    else if arg == "-v" || arg == "--verbose" { opts.verbose = true; }
    else if arg == "-x" || arg == "--one-file-system" { opts.skip_subdirectories = true; }
    // TODO: Ensure mutual exclusion with --preserve=context
    // TODO: Override -a and --preserve=all options
    else if arg == "-Z" { opts.selinux_context = true; }
    else if arg.starts_with("--context") {
        opts.selinux_context = true;
        if arg.len() < "--context=".len() {
            // TODO: Replace with proper error handling
            panic!("No context specified");
        }
        else {
            opts.context = arg["--context=".len()..].to_string();
        }
    }
    else {
        return -1;
    }
    */
    0
} // parse_argument

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
