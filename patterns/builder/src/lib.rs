use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
struct Computer {
    CPU: String,
    RAM: String,
    HDD: String,
    isGPUEnabled: bool,
    isBluetoothEnabled: bool,
}

impl Computer {
    pub fn new(builder: ComputerBuilder) -> Self {
        Computer {
            CPU: builder.CPU,
            RAM: builder.RAM,
            HDD: builder.HDD,
            isGPUEnabled: builder.isGPUEnabled,
            isBluetoothEnabled: builder.isBluetoothEnabled,
        }
    }
}


impl std::fmt::Display for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Computer Specifications:\nCPU: {}\nRAM: {}\nHDD: {}\nGPU Enabled: {}\nBluetooth Enabled: {}",
               self.CPU, 
               self.RAM, 
               self.HDD, 
               if self.isGPUEnabled { "Yes" } else { "No" },
               if self.isBluetoothEnabled { "Yes" } else { "No" }
        )
    }
}


#[derive(Debug, PartialEq, Eq)]
struct ComputerBuilder {
    CPU: String,
    RAM: String,
    HDD: String,
    isGPUEnabled: bool,
    isBluetoothEnabled: bool,
}

impl ComputerBuilder {
    pub fn default() -> Self {
        ComputerBuilder {
            CPU: "default-CPU".to_string(),
            RAM: "default-RAM".to_string(),
            HDD: "default-HDD".to_string(),
            isGPUEnabled: true,
            isBluetoothEnabled: false,
        }
    }

    pub fn setCPU(mut self, cpu: String) -> Self {
        self.CPU = cpu;
        self
    }
    pub fn setRAM(mut self, ram: String) -> Self {
        self.RAM = ram;
        self
    }
    pub fn setHDD(mut self, hdd: String) -> Self {
        self.HDD = hdd;
        self
    }
    pub fn enableGPU(mut self) -> Self {
        self.isGPUEnabled = true;
        self
    }
    pub fn disableGPU(mut self) -> Self {
        self.isGPUEnabled = false;
        self
    }
    pub fn enableBluetooth(mut self) -> Self {
        self.isBluetoothEnabled = true;
        self
    }
    pub fn disableBluetooth(mut self) -> Self {
        self.isBluetoothEnabled = false;
        self
    }
}

// run it with `cargo test -- --nocapture`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let cb = ComputerBuilder::default();
        let com = Computer::new(cb);
        println!("{com}");
        println!("-----\n\n");
        let cb = ComputerBuilder::default()
            .setCPU("intel core i9".to_string())
            .setRAM("32GB kingston".to_string())
            .setHDD("1TB Western Digital".to_string())
            .disableGPU()
            .enableBluetooth();

        let com = Computer::new(cb);
        println!("{com}");
        println!("-----\n\n");
    }
}
