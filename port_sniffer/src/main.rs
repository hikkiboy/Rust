use std::env;
fn main() {
    let args: Vec<String> = env :: args().collect();
    let program = args[0].clone();
}

/*ip_sniffer.exe -h
ip_sniffer.exe -j 
ip_sniffer.exe IP */