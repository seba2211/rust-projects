// 7.4 from rust book

/* 
* // To compress this expression:
* use std::cmp::Ordering;
* use std::io;
*  // Use this:
* use std::{cmp::Ordering, io};
*/

use std::{cmp::Ordering, io};

/* 
* // Even the 'self' keyword could be used
* use std::io;
* use std::io::Write;
*  // Into:
* use std::io::{self, Write};
*/

use std::io::{self, Write};

// The glob operator
use std::collections::*;
// This use statement brings all public items defined in std::collections into the current scope.

fn main() {
    // This code does not compile! (multiple module inclusion)
}