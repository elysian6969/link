use link::Library;

type MallocFn = unsafe extern "C" fn(len: usize) -> *mut u8;
type FreeFn = unsafe extern "C" fn(ptr: *mut u8) -> i32;

fn main() {
    unsafe {
        let library = Library::load("libc.so.6").unwrap();
        let malloc: MallocFn = library.symbol("malloc").unwrap();
        let free: FreeFn = library.symbol("free").unwrap();

        println!("malloc = {malloc:?}");
        println!("free = {free:?}");

        let foo = malloc(4).cast::<i32>();

        foo.write(5);

        println!("foo = {:?}", foo.read());

        free(foo.cast());
    }
}
