use sysinfo::{ProcessorExt, System, SystemExt};

pub struct TestStruct {
    sys_info: System,
    pub num_cores: u8,
    cpu_usage: f32,
    aves: Vec<f32>,
}

impl TestStruct {
    // pub fn add(&mut self) -> f32 {
    //     self.num += 2.3;
    //     self.num
    // }
}

impl Default for TestStruct {
    fn default() -> TestStruct {
        TestStruct {
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
            cpu_usage: 0.0,
            aves: vec![0.0, 0.0, 0.0, 0.0, 0.0],
        }
    }
}
