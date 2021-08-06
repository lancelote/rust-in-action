use rand::random;

static mut ERROR: isize = 0;

struct File;

#[allow(unused_variables)]
fn read(file: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

fn main() {
    let file = File;
    let mut buffer = vec![];

    read(&file, &mut buffer);

    unsafe {
        if ERROR != 0 {
            panic!("An error has occurred!");
        }
    }
}
