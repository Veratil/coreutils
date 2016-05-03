
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

