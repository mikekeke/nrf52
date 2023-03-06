use std::time::Duration;

fn main() {
    println!("testing");
    let ports = serialport::available_ports().expect("No ports found!");
    println!("ports {:?}", ports);
    for p in ports {
        println!("{}", p.port_name);
    }

    // ! sudo chown mike /dev/ttyACM0   
    let mut port = serialport::new("/dev/ttyACM0", 115_200)
        .timeout(Duration::from_millis(100000))
        .open()
        .expect("Failed to open port");

    println!("writing");
    let w_res = port.write(b"tests00dsdsdsdsd usbdddddddddddddddddddddddddddddddddddddddddddddddddddddddddd");
    println!("writing {:?}", w_res);

    
    // println!("reading");
    // let mut buf: Vec<u8> = vec![0; 64];
    // port.read(buf.as_mut_slice()).expect("Found no data!");
    
    // println!("buf");
    // println!("Buf: {:?}", buf)
}
