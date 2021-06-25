// ------------------------------------------------------------
// esponenziale.rs
//
// Copyright Andrea G. Quaglia, 2021
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// ----------------------------------------------------------------

// Usage: enter the initial value, the exponent factor, the maximum time

use std::io;

fn main() {
    println!("Demonstration of exponent growth.");

    println!("Enter the initial value:");

    let initial: f64;
    let multipl: f64;
    let mut max_recurs: u16;
    let mut riga = String::new();

    io::stdin()
        .read_line(&mut riga)
        .expect("Error reading");

    initial = riga.trim().parse().expect("Please type a valid number");

    println!("Enter the exponential factor:");
    riga.clear();
    io::stdin()
        .read_line(&mut riga)
        .expect("Error reading");
    multipl = riga.trim().parse().expect("Please type a valid number");

    println!("Enter the iteration limit:");
    riga.clear();
    io::stdin()
        .read_line(&mut riga)
        .expect("Error reading");
    max_recurs = riga.trim().parse().expect("Please type a valid number");
    if max_recurs <= 0 {
        max_recurs = 15;
    }

    println!("test: initial {}, multipl {}, final = {}", initial, multipl, expon(initial, multipl, max_recurs));

}


fn expon(number: f64, multipl: f64, limit: u16) -> f64 {
    if limit > 0 {
        let calc: f64 = number * multipl;
        expon(calc, multipl, limit-1)
    }
    else
    {
        return number * multipl;
    }

}