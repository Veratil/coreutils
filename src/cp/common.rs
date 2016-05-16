
use std;

// default mode,ownership,timestamps
#[derive(Debug)]
pub struct PreserveType {
    pub mode: bool, // will default true
    pub ownership: bool, // will default true
    pub timestamps: bool, // will default true
    pub links: bool,
    pub security_context: bool,
    pub xattr: bool,
}
impl PreserveType {
    pub fn new() -> PreserveType {
        PreserveType {
            mode: false,
            ownership: false,
            timestamps: false,
            links: false,
            security_context: false,
            xattr: false,
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
pub enum BackupType {
    // Never make backups
    NoBackups, // none, off
    // Always make numbered backups
    Numbered, // numbered, t
    // Make numbered backups of files that already have them, simple backups of others
    NumberedExisting, // existing, nil
    // Always make simple backups. Please note 'never' is not to be confused with 'none'.
    Simple, // simple, never
}
pub static BACKUP_OPTIONS: &'static str =
r#"  - 'none', 'off'
  - 'simple', 'never'
  - 'existing', 'nil'
  - 'numbered', 't'"#;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReflinkType {
    Always, // default
    Auto,
    Never, // fallback of standard copy
}
pub static REFLINK_OPTIONS: &'static str =
r#"  - 'always'
  - 'auto'"#;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SparseType {
    Always,
    Auto, // default
    Never,
//    Unused, // type seen in C
}
pub static SPARSE_OPTIONS: &'static str =
r#"  - 'always'
  - 'auto'
  - 'never'"#;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Interactive {
    AlwaysYes,
    AlwaysNo,
    AskUser,
    Unspecified,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DereferenceSymlink {
    Undefined,
    Never, // -P
    CommandLineArguments, // -H
    Always, // -L
}

#[derive(Debug)]
pub struct CpOptions {
    // XXX: Options that are in cp.c are moved here for ease of visibility
    pub make_backups:               bool,
    pub copy_contents:              bool,
    pub parents_option:             bool,
    pub remove_trailing_slashes:    bool,
    pub backup_suffix_string:       String,
    pub target_directory:           String,
    pub no_target_directory:        bool,
    pub scontext:                   String,
    // *** End moved options
    pub backup_type:                BackupType,
    // How to handle symlinks in the source
    pub dereference:                DereferenceSymlink,
    // Determine whether to prompt before removing existing destination files
    // Work differently depending on whether move_mode is set
    pub interactive:                Interactive,
    // Control creation of sparse files
    pub sparse_mode:                SparseType,
    // Set the mode of the destination file to exactly this value if SET_MODE is nonzero
    pub mode:                       u32,
    // If true, copy all files except (directories, and if not dereferencing them, symbolic links)
    //  as if they were regular files
    pub copy_as_regular:            bool,
    // If true, remove each existing destination nondirectory before trying to open it
    pub unlink_dest_before_opening: bool,
    // If true, first try to open each existing destination nondirectory, then if open fails
    //  unlink and try again.
    //  This option must be set for 'cp -f', in case the destination file exists when the open
    //  is attempted.
    pub unlink_dest_after_failed_open:  bool,
    // If true, create hard links instead of copying files
    pub hard_link:                      bool,
    // If true, rather than copying, first attempt to use rename. If that fails then resort to copying
    pub move_mode:                      bool,
    // Whether this process has appropriate priviledges to chown a file
    //  whose owner is not the effective user ID
    pub chown_priviledges:              bool,
    // Whether this process has appropriate priviledges to do the following operations on a file
    //  even when it is owned by some other user: set the files atime, mtime, mode, or ACL;
    //  remove or rename an entry in the file even through it is a sticky directory, or to mount
    //  on the file.
    pub owner_priviledges:              bool,
    // If true, when copying recursively, skip any subdirectories that are on different
    //  filesystems from the one we started on
    pub one_file_system:                bool,
    // Preserve options, see PreserveType; accessed by replacing _ with .
    // contains: preserve_ownership
    //           preserve_mode
    //           preserve_timestamps
    //           preserve_links
    //           preserve_security_context
    //           preserve_xattr
    pub preserve:                       PreserveType,
    pub explicit_no_preserve_mode:      bool,
    // If true, attempt to set specified security context
    pub set_security_context:           bool,
    // Optionally don't copy the data, either with CoW relink files or explicitly with the
    //  --attributes-only option
    pub data_copy_required:             bool,
    // If true and mode,ownership,timestamps,links file attributes cannot be applied to destination
    //  file, treat it as a failure and return nonzero immediately. E.g. for cp -p this must be true
    pub require_preserve:               bool,
    // Useful only when preserve_context is true.
    //  If true, a failed attempt to preserve file's security context propagates failure "out"
    //   to the caller, along with full diagnostics.
    //  If false, a failure to preserve file's security context does not change the invoking
    //   applications exit status, but may output diagnostics.
    //  For example, with 'cp --preserve=context' this flag is "true",
    //   while with 'cp --preserve=all' or 'cp -a', it is "false"
    pub require_preserve_context:       bool,
    // Useful only when preserve_xattr is true
    //  If true, a failed attempt to preserve file's extended attributes propagates failure "out"
    //   to the caller, along with full diagnostics.
    //  If false, a failure to preserve file's extended attributes does not change the invoking
    //   applications exit status, but may output diagnostics
    //  For example, with 'cp --preserve=xattr' this flag is "true",
    //   while with 'cp --preserve=all' or 'cp -a', it is "false"
    pub require_preserve_xattr:         bool,
    // This allows us to output warnings in cases 2 and 4 below,
    //  while being quiet for case 1 (when reduce_diagnostics is true)
    //   1. cp -a                       try to copy xattrs with no errors
    //   2. cp --preserve=all           copy xattrs with all but ENOTSUP warnings
    //   3. cp --preserve=xattr,context copy xattrs with all errors
    //   4. mv                          copy xattrs with all but ENOTSUP warnings
    pub reduce_diagnostics:             bool,
    // If true, copy directories recursively and copy special files as themselves rather
    //  than copying their contents
    pub recursive:                      bool,
    // If true, set file mode to value of MODE. Otherwise, set it based on current umask
    //  modified by UMASK_KILL
    pub set_mode:                       bool,
    // If true, create symbolic links instead of copying files. Create destination directories as usual
    pub symbolic_link:                  bool,
    // If true, do not copy a nondirectory that has an existing destination with the same name
    //  or newer modification time
    pub update:                         bool,
    // If true, display the names of the files before copying them
    pub verbose:                        bool,
    // If true, stdin is a tty
    pub stdin_tty:                      bool,
    // If true, open a dangling destination symlink when not in move_mode. Otherwise copy_reg gives
    //  a diagnostic (it refuses to write through such a symlink) and returns false
    pub open_dangling_dest_symlink:     bool,
    // Control creation of CoW files
    pub reflink_mode:                   ReflinkType,
    // pub dest_info: HashTable,
    // pub src_info:  HashTable
}
impl CpOptions {
    pub fn new() -> CpOptions {
        CpOptions {
            make_backups: false,
            copy_contents: false,
            parents_option: false,
            remove_trailing_slashes: false,
            backup_suffix_string: String::new(),
            target_directory: String::new(),
            no_target_directory: false,
            scontext: String::new(),

            backup_type: BackupType::NoBackups,
            dereference: DereferenceSymlink::Undefined,
            interactive: Interactive::Unspecified,
            sparse_mode: SparseType::Auto,
            mode: 0,
            copy_as_regular: true,
            unlink_dest_before_opening: false,
            unlink_dest_after_failed_open: false,
            hard_link: false,
            move_mode: false,
            // FIXME: chown_priviledges and owner_priviledges depend on getppriv()/priv_ismember()
            chown_priviledges: false,
            owner_priviledges: false,
            one_file_system: false,
            preserve: PreserveType::new(),
            explicit_no_preserve_mode: false,
            set_security_context: false, /* -Z, set sys default context */
            data_copy_required: true,
            require_preserve: false,
            require_preserve_context: false, /* --preserve=context */
            require_preserve_xattr: false,
            reduce_diagnostics: false,
            recursive: false,
            set_mode: false,
            symbolic_link: false,
            update: false,
            verbose: false,
            stdin_tty: false, /* Not used */
            // FIXME: getenv("POSIXLY_CORRECT") != NULL
            open_dangling_dest_symlink: false,
            reflink_mode: ReflinkType::Never,
        }
    }
}

pub static VERSION: &'static str = env!("CARGO_PKG_VERSION");
//static AUTHORS: [&'static str; 2] = ["Jordy Dickinson", "Kevin Zander"];


pub fn print_missing_argument(forarg: &str) {
    print_cp_error(format!("option requires an argument -- '{}'", forarg).as_str());
}

pub fn print_bad_argument(badt: &str, arg: &str, forarg: &str, explain: &str) {
    print_cp_error(format!("{} argument '{}' for '{}'\nValid arguments are:\n{}",
        badt, arg, forarg, explain).as_str());
}

pub fn print_ambiguous_argument(ambarg: &str, forarg: &str, explain: &str) {
    print_bad_argument("ambiguous", ambarg, forarg, explain);
}

pub fn print_invalid_argument(invarg: &str, forarg: &str, explain: &str) {
    print_bad_argument("invalid", invarg, forarg, explain);
}

pub fn print_cp_error(msg: &str) {
    println!("{0}: {1}", std::env::args().nth(0).unwrap(), msg);
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
