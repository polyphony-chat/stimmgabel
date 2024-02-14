// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use rand::prelude::Rng;

pub fn run(_port: u16) {
    println!("Hello, world!");
}

pub(super) fn random_name() -> String {
    let names = [
        "Cottage",
        "Baguette",
        "Xenia",
        "Mechanist",
        "Legibly",
        "Scrounger",
        "Unwed",
        "Wed",
        "Shady",
        "Amusing",
        "Deduce",
        "Deduct",
        "Excused",
        "Infer",
        "Rusty",
        "Okay",
    ];
    let mut numbers = Vec::<u8>::with_capacity(4);
    let mut rng = rand::thread_rng();
    for _ in 0..4 {
        numbers.push(rng.gen_range(0..=9))
    }
    format!(
        "{}{}",
        names.get(rng.gen_range(0..=15)).unwrap(),
        numbers.iter().map(|x| x.to_string()).collect::<String>()
    )
}
