use sysinfo::{ProcessorExt, System, SystemExt};

pub struct SystemInfo {
    sys_info: System,
    num_cores: u8,
    aves: Vec<f32>,
}

impl SystemInfo {
    // Get current cpu usage (average of all cores) & return average of last 5 usage values.
    pub fn cpu_usage(&mut self) -> f32 {
        self.sys_info.refresh_all();

        let mut total: f32 = 0.0;

        for processor in self.sys_info.get_processors() {
            total += processor.get_cpu_usage();
        }

        let new_ave = total / self.num_cores as f32;

        self.add_to_queue(new_ave);

        return self.array_ave();
    }

    fn array_ave(&mut self) -> f32 {
        let count: usize = self.aves.iter().count();
        let mut total: f32 = 0.0;

        for i in 0..count {
            total += self.aves[i];
        }

        return total / count as f32;
    }

    fn add_to_queue(&mut self, new_elem: f32) {
        self.aves.push(new_elem); // add element to end of vector
        self.aves.remove(0); // remove element at start of vector
    }
}

impl Default for SystemInfo {
    fn default() -> SystemInfo {
        SystemInfo {
            sys_info: sysinfo::System::new_all(),
            num_cores: {
                let system = sysinfo::System::new_all();
                let mut num: u8 = 0;

                // Count virtual cores.
                for _processor in system.get_processors() {
                    num += 1;
                }

                num
            },
            aves: vec![0.0; 5],
        }
    }
}
