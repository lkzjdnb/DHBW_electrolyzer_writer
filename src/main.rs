use clap::Parser;
use core::time;
use log::{debug, error, info, warn};
use modbus_device::{register::Register, ModbusConnexion, ModbusDevice};
use rand::Rng;
use std::{fs::File, thread};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = "127.0.0.1:502",
        help = "The device address",
        long_help = "The device ip address as a parseable string ex : 127.0.0.1:502"
    )]
    remote: String,

    #[arg(
        long,
        default_value = "input_registers.json",
        help = "Path to the json file containing the registers definition"
    )]
    input_register_path: String,

    #[arg(
        long,
        default_value = "holding_registers.json",
        help = "Path to the json file containing the registers definition"
    )]
    holding_register_path: String,
}

fn main() -> () {
    env_logger::init();

    let args = Args::parse();

    let electrolyzer_input_registers_json = match File::open(&args.input_register_path) {
        Ok(file) => file,
        Err(err) => panic!(
            "Could not open the file containing the input registers definition : {0} ({err:?})",
            &args.input_register_path
        ),
    };
    let electrolyzer_holding_registers_json = match File::open(&args.holding_register_path) {
        Ok(file) => file,
        Err(err) => panic!(
            "Could not open the file containing the holding registers definition : {0} ({err:?})",
            &args.holding_register_path
        ),
    };

    let electrolyzer_address = match args.remote.parse() {
        Ok(addr) => addr,
        Err(err) => panic!("Invalid remote address entered {0} ({err})", args.remote),
    };

    let mut electrolyzer = ModbusDevice {
        ctx: match modbus_device::connect(electrolyzer_address) {
            Ok(ctx) => ctx,
            Err(err) => panic!("Error connecting to device {electrolyzer_address} ({err})"),
        },
        input_registers: match modbus_device::get_defs_from_json(electrolyzer_input_registers_json)
        {
            Ok(registers) => registers,
            Err(err) => panic!("Could not load input registers definition from file ({err})"),
        },
        holding_registers: match modbus_device::get_defs_from_json(
            electrolyzer_holding_registers_json,
        ) {
            Ok(registers) => registers,
            Err(err) => panic!("Could not load holding registers definition from file ({err})"),
        },
    };

    let production_register: Register = electrolyzer
        .get_holding_register_by_name("ProductionRate[%]".to_string())
        .unwrap()
        .clone();

    let mut rng = rand::thread_rng();

    loop {
        // let val: f32 = rng.gen::<f32>() * 40.0 + 60.0;
        let val = 70.0;
        info!("Setting production to {val}%");
        electrolyzer.write_holding_register(production_register.clone(), val.into());

        thread::sleep(time::Duration::from_secs(10));
    }
}
