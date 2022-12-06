use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    // For part 1:
    // const MARKER_LEN: usize = 4;
    // For part 2:
    const MARKER_LEN: usize = 14;

    let mut last4 = vec![];
    let mut index = 0u32;

    for br in io::BufReader::new(&file).bytes() {
        match br {
            Ok(b) => {
                if last4.len() < MARKER_LEN {
                    last4.push(b as char);
                } else {
                    let mut check = last4.clone();
                    check.sort();
                    check.dedup();

                    if check.len() == MARKER_LEN {
                        // found!
                        break;
                    } else {
                        last4.remove(0);
                        last4.push(b as char);
                    }
                }

                index += 1;
            }
            Err(_) => {
                break;
            }
        }
    }

    println!("Result: {}", index);

    Ok(())
}
