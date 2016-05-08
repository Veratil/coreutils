
use std;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum BackupMethod {
    // Never make backups
    none,
    off,
    // Always make numbered backups
    numbered,
    t,
    // Make numbered backups of files that already have them, simple backups of others
    existing, // default
    nil,
    // Always make simple backups. Please note 'never' is not to be confused with 'none'.
    simple,
    never,
}
pub static BACKUP_OPTIONS: &'static str =
r#"  - 'none', 'off'
  - 'simple', 'never'
  - 'existing', 'nil'
  - 'numbered', 't'"#;

// default mode,ownership,timestamps
#[derive(Debug)]
pub struct PreserveAttributes {
    pub mode: bool, // default true
    pub ownership: bool, // default true
    pub timestamps: bool, // default true
    pub links: bool,
    pub context: bool,
    pub xattr: bool,
    pub all: bool,
}
impl PreserveAttributes {
    // default mode handled with argument parsing
    pub fn new() -> PreserveAttributes {
        PreserveAttributes {
            mode: true,
            ownership: true,
            timestamps: true,
            links: false,
            context: false,
            xattr: false,
            all: false
        }
    }
    // used when preserve arguments are present
    pub fn clean() -> PreserveAttributes {
        PreserveAttributes {
            mode: false,
            ownership: false,
            timestamps: false,
            links: false,
            context: false,
            xattr: false,
            all: false
        }
    }
}
pub static PRESERVE_OPTIONS: &'static str =
r#"  - ‘mode’
  - ‘timestamps’
  - ‘ownership’
  - ‘links’
  - ‘context’
  - ‘xattr’
  - ‘all’"#;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum ReflinkWhen {
    always, // default
    auto,
}
pub static REFLINK_OPTIONS: &'static str =
r#"  - 'always'
  - 'auto'"#;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SparseWhen {
    always,
    auto, // default
    never,
}
pub static SPARSE_OPTIONS: &'static str =
r#"  - 'always'
  - 'auto'
  - 'never'"#;

#[derive(Debug)]
pub struct Mode {
    // equivalent to -dR --preserve=all with reduced diagnostics
    pub archive:                bool,               // -a, --archive
    pub attributes_only:        bool,               // --attributes-only
    pub backup:                 bool,               // -b, --backup[=method]
    pub backup_method:          BackupMethod,       // backup method, default existing
    pub copy_contents:          bool,               // --copy-contents
    // equivalent to --no-dereference --preserve=links
    pub copy_as_symlinks:       bool,               // -d
    // independent of -i or --interactive, neither cancels effect of other
    // ignored when -n or --no-clobber is used
    pub force:                  bool,               // -f, --force
    pub copy_real_file:         bool,               // -H
    // overrides previous -n option
    pub interactive:            bool,               // -i, --interactive
    pub copy_as_hardlink:       bool,               // -l, --link
    pub dereference:            bool,               // -L, --dereference
    // overrides previous -i option
    // mutually exclusive with -b or --backup option
    pub no_clobber:             bool,               // -n, --no-clobber
    pub no_dereference:         bool,               // -P, --no-dereference
    // with no attribute_list defaults to --preserve=mode,ownership,timestamps
    pub preserve:               bool,               // -p, --preserve[=attribute_list]
    pub preserve_attributes:    PreserveAttributes, // default mode,ownership,timestamps
    pub no_preserve:            bool,               // --no-preserve=attribute_list
    pub nopreserve_attributes:  PreserveAttributes, // list for no-preserve
    pub parents:                bool,               // --parents
    // Recursive is a beast, setting certain defaults and following certain rules
    pub recursive:              bool,               // -R, -r, --recursive
    // overridden by the --link, --symbolic-link, and --attributes-only options
    // TODO: How??
    pub reflink:                bool,               // --reflink[=when]
    pub reflink_when:           ReflinkWhen,        // default always
    pub remove_destination:     bool,               // --remove-destintation
    pub sparse:                 bool,               // --sparse=when
    pub sparse_when:            SparseWhen,         // default auto
    pub strip_trailing_slashes: bool,               // --strip-trailing-slashes
    pub symbolic_link:          bool,               // -s, --symbolic-link
    pub suffix:                 String,             // -S suffix, --suffix=suffix
    // -t and -T are mutually exclusive
    pub target_directory:       String,             // -t directory, --target-directory=directory
    pub no_target_directory:    bool,               // -T, --no-target-directory
    pub update:                 bool,               // -u, --update
    pub verbose:                bool,               // -v, --verbose
    pub skip_subdirectories:    bool,               // -x, --one-file-system
    // mutually exclusive with --preserve=context
    // overrides -a and --preserve=all options
    pub selinux_context:        bool,               // -Z, --context[=context]
    pub context:                String,             // context for selinux
}
impl Mode {
    pub fn new() -> Mode {
        Mode {
            archive: false,
            attributes_only: false,
            backup: false,
            backup_method: BackupMethod::existing,
            copy_contents: false,
            copy_as_symlinks: false,
            force: false,
            copy_real_file: false,
            interactive: false,
            copy_as_hardlink: false,
            dereference: false,
            no_clobber: false,
            no_dereference: false,
            preserve: false,
            preserve_attributes: PreserveAttributes::new(),
            no_preserve: false,
            nopreserve_attributes: PreserveAttributes::new(),
            parents: false,
            recursive: false,
            reflink: false,
            reflink_when: ReflinkWhen::always,
            remove_destination: false,
            sparse: false,
            sparse_when: SparseWhen::auto,
            strip_trailing_slashes: false,
            symbolic_link: false,
            suffix: String::new(),
            target_directory: String::new(),
            no_target_directory: false,
            update: false,
            verbose: false,
            skip_subdirectories: false,
            selinux_context: false,
            context: String::new(),
        }
    }
}


pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");
//static AUTHORS: [&'static str; 2] = ["Jordy Dickinson", "Kevin Zander"];


pub fn print_missing_destination_file(file: &str) {
    println!("{0}: missing destination file operand after '{1}'",
        std::env::args().nth(0).unwrap(), file);
}

pub fn print_missing_files() {
    println!("{0}: missing file operand",
        std::env::args().nth(0).unwrap());
}

pub fn print_missing_argument(forarg: &str) {
    println!("{0}: option requires an argument -- '{1}'",
        std::env::args().nth(0).unwrap(), forarg);
}

pub fn print_bad_argument(badt: &str, arg: &str, forarg: &str, explain: &str) {
    println!("{0}: {1} argument '{2}' for '{3}'\nValid arguments are:\n{4}",
        std::env::args().nth(0).unwrap(), badt, arg, forarg, explain);
}

pub fn print_ambiguous_argument(ambarg: &str, forarg: &str, explain: &str) {
    print_bad_argument("ambiguous", ambarg, forarg, explain);
}

pub fn print_invalid_argument(invarg: &str, forarg: &str, explain: &str) {
    print_bad_argument("invalid", invarg, forarg, explain);
}

pub fn print_cp_help() {
    println!("Try '{0} --help' for more information.", std::env::args().nth(0).unwrap());
}

pub fn print_version() {
    println!("{} {}", std::env::args().nth(0).unwrap(), VERSION);
}

pub fn print_help() {
    let msg = format!(
r#"Usage: {0} [OPTION]... [-T] SOURCE DEST
  or:  {0} [OPTION]... SOURCE... DIRECTORY
  or:  {0} [OPTION]... -t DIRECTORY SOURCE...
Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.

Mandatory arguments to long options are mandatory for short options too.
  -a, --archive                same as -dR --preserve=all
      --attributes-only        don't copy the file data, just the attributes
      --backup[=CONTROL]       make a backup of each existing destination file
  -b                           like --backup but does not accept an argument
      --copy-contents          copy contents of special files when recursive
  -d                           same as --no-dereference --preserve=links
  -f, --force                  if an existing destination file cannot be
                                 opened, remove it and try again (this option
                                 is ignored when the -n option is also used)
  -i, --interactive            prompt before overwrite (overrides a previous -n
                                 option)
  -H                           follow command-line symbolic links in SOURCE
  -l, --link                   hard link files instead of copying
  -L, --dereference            always follow symbolic links in SOURCE
  -n, --no-clobber             do not overwrite an existing file (overrides
                                 a previous -i option)
  -P, --no-dereference         never follow symbolic links in SOURCE
  -p                           same as --preserve=mode,ownership,timestamps
      --preserve[=ATTR_LIST]   preserve the specified attributes (default:
                                 mode,ownership,timestamps), if possible
                                 additional attributes: context, links, xattr,
                                 all
  -c                           deprecated, same as --preserve=context
      --no-preserve=ATTR_LIST  don't preserve the specified attributes
      --parents                use full source file name under DIRECTORY
  -R, -r, --recursive          copy directories recursively
      --reflink[=WHEN]         control clone/CoW copies. See below
      --remove-destination     remove each existing destination file before
                                 attempting to open it (contrast with --force)
      --sparse=WHEN            control creation of sparse files. See below
      --strip-trailing-slashes  remove any trailing slashes from each SOURCE
                                 argument
  -s, --symbolic-link          make symbolic links instead of copying
  -S, --suffix=SUFFIX          override the usual backup suffix
  -t, --target-directory=DIRECTORY  copy all SOURCE arguments into DIRECTORY
  -T, --no-target-directory    treat DEST as a normal file
  -u, --update                 copy only when the SOURCE file is newer
                                 than the destination file or when the
                                 destination file is missing
  -v, --verbose                explain what is being done
  -x, --one-file-system        stay on this file system
  -Z                           set SELinux security context of destination
                                 file to default type
      --context[=CTX]          like -Z, or if CTX is specified then set the
                                 SELinux or SMACK security context to CTX
      --help     display this help and exit
      --version  output version information and exit

By default, sparse SOURCE files are detected by a crude heuristic and the
corresponding DEST file is made sparse as well.  That is the behavior
selected by --sparse=auto.  Specify --sparse=always to create a sparse DEST
file whenever the SOURCE file contains a long enough sequence of zero bytes.
Use --sparse=never to inhibit creation of sparse files.

When --reflink[=always] is specified, perform a lightweight copy, where the
data blocks are copied only when modified.  If this is not possible the copy
fails, or if --reflink=auto is specified, fall back to a standard copy.

The backup suffix is '~', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.
The version control method may be selected via the --backup option or through
the VERSION_CONTROL environment variable.  Here are the values:

  none, off       never make backups (even if --backup is given)
  numbered, t     make numbered backups
  existing, nil   numbered if numbered backups exist, simple otherwise
  simple, never   always make simple backups

As a special case, cp makes a backup of SOURCE when the force and backup
options are given and SOURCE and DEST are the same name for an existing,
regular file.
"#,
        std::env::args().nth(0).unwrap());
    println!("{}", msg);
}
