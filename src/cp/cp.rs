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

use common::{Mode};

mod parser;
mod common;

//use std::fs;
//use std::io::{ErrorKind, Result, Write};
//use std::path::Path;
//use uucore::fs::{canonicalize, CanonicalizeMode};

pub fn uumain(args: Vec<String>) -> i32 {
    let mut opts: Mode = Mode::new();


    let (ret, sdargs) = parser::parse_args(args, &mut opts);

    println!("{:?}", opts);
    println!("{:?}", ret);
    println!("{:?}", sdargs);

    //copy(opts);

    0
} // uumain()


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
