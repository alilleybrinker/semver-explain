use semver::{Comparator, Op, VersionReq};
use std::process::exit;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();

    let version = match VersionReq::parse(&opt.version_req) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: failed to parse version with error '{}'", e);
            fail();
        }
    };

    for comparator in &version.comparators {
        explain_comparator(comparator);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "semver-explain")]
struct Opt {
    /// semantic versioning requirement to explain
    #[structopt(name = "VERSION_REQ")]
    version_req: String,
}

fn explain_comparator(c: &Comparator) {
    match c.op {
        Op::Exact => explain_exact(c),
        Op::Greater => explain_greater(c),
        Op::GreaterEq => explain_greater_eq(c),
        Op::Less => explain_less(c),
        Op::LessEq => explain_less_eq(c),
        Op::Tilde => explain_tilde(c),
        Op::Caret => explain_caret(c),
        Op::Wildcard => explain_wildcard(c),
        unknown => {
            eprintln!("error: unknown comparator encountered: '{:?}'", unknown);
            fail();
        }
    }
}

fn explain_exact(c: &Comparator) {
    match (c.major, c.minor, c.patch) {
        (maj, Some(min), Some(pat)) => println!("={}.{}.{}", maj, min, pat),
        (maj, Some(min), None) => println!(">={}.{}.0, <{}.{}.0", maj, min, maj, bump(min)),
        (maj, None, _) => println!(">={}.0.0, <{}.0.0", maj, bump(maj)),
    }
}

fn explain_greater(c: &Comparator) {
    match (c.major, c.minor, c.patch) {
        (maj, Some(min), Some(pat)) => println!(">{}.{}.{}", maj, min, pat),
        (maj, Some(min), None) => println!(">={}.{}.0", maj, bump(min)),
        (maj, None, _) => println!(">={}.0.0", bump(maj)),
    }
}

fn explain_greater_eq(c: &Comparator) {
    match (c.major, c.minor, c.patch) {
        (maj, Some(min), Some(pat)) => println!(">={}.{}.{}", maj, min, pat),
        (maj, Some(min), None) => println!(">={}.{}.0", maj, min),
        (maj, None, _) => println!(">={}.0.0", maj),
    }
}

fn explain_less(c: &Comparator) {
    match (c.major, c.minor, c.patch) {
        (maj, Some(min), Some(pat)) => println!("<{}.{}.{}", maj, min, pat),
        (maj, Some(min), None) => println!("<{}.{}.0", maj, min),
        (maj, None, _) => println!("<{}.0.0", maj),
    }
}

fn explain_less_eq(c: &Comparator) {
    match (c.major, c.minor, c.patch) {
        (maj, Some(min), Some(pat)) => println!("<={}.{}.{}", maj, min, pat),
        (maj, Some(min), None) => println!("<{}.{}.0", maj, bump(min)),
        (maj, None, _) => println!("<{}.0.0", bump(maj)),
    }
}

fn explain_tilde(c: &Comparator) {
    match (c.major, c.minor, c.patch) {
        (maj, Some(min), Some(pat)) => {
            println!(">={}.{}.{}, <{}.{}.0", maj, min, pat, maj, bump(min))
        }
        (maj, Some(min), None) => println!(">={}.{}.0, <{}.{}.0", maj, min, maj, bump(min)),
        (maj, None, _) => println!(">={}.0.0, <{}.0.0", maj, bump(maj)),
    }
}

fn explain_caret(c: &Comparator) {
    match (c.major, c.minor, c.patch) {
        (maj, Some(min), Some(pat)) if maj > 0 => {
            println!(">={}.{}.{}, <{}.0.0", maj, min, pat, bump(maj))
        }
        (maj, Some(min), Some(pat)) if maj == 0 && min > 0 => {
            println!(">=0.{}.{}, <0.{}.0", min, pat, bump(min))
        }
        (maj, Some(min), Some(pat)) if maj == 0 && min == 0 => println!("=0.0.{}", pat),

        (maj, Some(min), None) if maj > 0 => {
            println!(">={}.{}.0, <{}.0.0", maj, min, bump(maj))
        }
        (maj, Some(min), None) if maj == 0 && min > 0 => {
            println!(">=0.{}.0, <0.{}.0", min, bump(min))
        }
        (maj, Some(min), None) if maj == 0 && min == 0 => {
            println!("=0.0.0")
        }
        (maj, None, _) => println!(">={}.0.0, <{}.0.0", maj, bump(maj)),
        (_maj, Some(_min), Some(..) | None) => {
            eprintln!("error: unrecognized bound {}", c);
            fail();
        }
    }
}

fn explain_wildcard(c: &Comparator) {
    match (c.major, c.minor, c.patch) {
        (_maj, Some(_min), Some(_pat)) => {
            eprintln!("error: can't have a wildcard where every version part is specified");
            fail();
        }
        (maj, Some(min), None) => println!(">={}.{}.0, <{}.{}.0", maj, min, maj, bump(min)),
        (maj, None, _) => println!(">={}.0.0, <{}.0.0", maj, bump(maj)),
    }
}

fn bump(version_num: u64) -> u64 {
    version_num + 1
}

fn fail() -> ! {
    exit(EXIT_FAILURE);
}

const EXIT_FAILURE: i32 = 1;
