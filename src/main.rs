mod error;
mod pfs0;
mod reader;

fn main() {
    // Extract and list contents of (NSP - Nintendo Submission Package)
    pfs0::PFS0::open_file("../roms/arcade-archives-darius-II.nsp").unwrap();
}
