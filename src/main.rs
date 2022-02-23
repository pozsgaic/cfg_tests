// Explore the cfg conditional parameter

#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
fn echo () {
    println!("x86_64-macos");
}

#[cfg(not(all(target_arch = "x86_64", target_os = "macos")))]
fn echo () {
    println!("OTHER");
}

//#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
//fn echo () {
//    println!("aarch64-macos");
//}

//#[cfg(not(all(target_arch = "aarch64", target_os = "macos")))]
//fn echo () {
//    println!("x86-64-linux");
//}


fn main() {
    println!("Starting!");
    echo();
}
