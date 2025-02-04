// Copyright 2018 Guillaume Pinot (@TeXitoi) <texitoi@texitoi.eu>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "make-cookie")]
struct MakeCookie {
    #[structopt(short)]
    s: String,

    #[structopt(subcommand, long)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    #[structopt(name = "pound")]
    /// Pound acorns into flour for cookie dough.
    Pound { acorns: u32 },

    Sparkle {
        #[structopt(short)]
        color: String,
    },
}

fn main() {}
