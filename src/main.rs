extern crate tomography;

fn main() {
    let misc = tomography::Misc::new();
    let fs = tomography::FileSystem::new();
    let mem = tomography::Memory::new();
    let cpu = tomography::Cpu::new();
    let net = tomography::Network::new();

    println!("Waiting 2 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("{:#?}", cpu.load());
    println!("{:#?}", cpu.loadavg());
    println!("{:#?}", net.interfaces());
    println!("{:#?}", misc.boot_time());
    println!("{:#?}", fs.all());
    println!("{:#?}", mem.ram());
    println!("{:#?}", mem.swap());

    cpu.close();
    net.close();
}
