extern crate tomography;

fn main() {
    let misc = tomography::Misc::new();
    let fs = tomography::FileSystem::new();
    let mem = tomography::Memory::new();
    let therm = tomography::Thermal::new();
    let power = tomography::Power::new();
    let cpu = tomography::Cpu::new();
    let net = tomography::Network::new();

    println!("Waiting 1 second...");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("{:#?}", cpu.load());
        println!("{:#?}", cpu.loadavg());
        println!("{:#?}", net.interfaces());
        println!("{:#?}", misc.boot_time());
        println!("{:#?}", fs.all());
        println!("{:#?}", mem.ram());
        println!("{:#?}", mem.swap());
        println!("{:#?}", therm.fans());
        println!("{:#?}", therm.cpus());
        println!("{:#?}", therm.custom("TA0P"));
        println!("{:#?}", power.sources());
    }

    // cpu.close();
    // net.close();
}
