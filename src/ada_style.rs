pub fn ada_begin() {
    println!("> begin Ada-like block");
}

pub fn ada_end() {
    println!("< end Ada-like block");
}

pub fn ada_block<T, F: FnOnce() -> T>(f: F) -> T {
    println!(":: entering Ada block");
    let val = f();
    println!(":: leaving Ada block");
    val
}

pub fn ada_procedure<F: FnOnce()>(name: &str, f: F) {
    println!("procedure {} is", name);
    f();
    println!("end {};", name);
}

pub fn ada_return<T>(value: T) -> T {
    println!("returning value...");
    value
}

pub fn ada_log<T: std::fmt::Debug>(label: &str, val: T) {
    println!("{} => {:?}", label, val);
}
