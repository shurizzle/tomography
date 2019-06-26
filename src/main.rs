extern crate tomography;

fn main() {
    let cpu = tomography::Cpu::new();
    let net = tomography::Network::new();

    println!("Waiting 2 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("{:#?}", cpu.load());
    println!("{:#?}", net.interfaces());

    cpu.close();
    net.close();
}
