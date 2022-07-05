use link::Library;

fn main() {
    unsafe {
        let library = Library::load("libc.so.6").unwrap();

        println!("library len = {:?}", library.bytes().len());
    }
}
