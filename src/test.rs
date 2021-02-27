pub struct TestStruct {
    pub num: f32,
}

impl TestStruct {
    pub fn add(&mut self) -> f32 {
        self.num += 2.3;
        self.num
    }
}

impl Default for TestStruct {
    fn default() -> TestStruct {
        TestStruct {
            num: 2.4,
        }
    }
}