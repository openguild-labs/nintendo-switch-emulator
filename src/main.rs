mod cpu;
mod err;
mod nca;

fn main() {
    // Extract and list contents of (NSP - Nintendo Submission Package)
    nca::pfs0::PFS0::open_file("../roms/arcade-archives-darius-II.nsp").unwrap();
}
