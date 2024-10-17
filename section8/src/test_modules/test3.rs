pub fn test_fn1() {
    println!("call test3::test_fn1()");
}

//
pub fn test_fn2() {
    println!("call test3::test_fn2()");
}

#[derive(Debug)]
pub struct PubTestStruct {
    pub val_1: i32,
    pub val_2: i32,
}

#[derive(Debug)]
pub struct PrvTestStructFields {
    val_1: i32,
    val_2: i32,
}
impl PrvTestStructFields {
    pub fn new(val_1: i32, val_2: i32) -> PrvTestStructFields {
        PrvTestStructFields { val_1, val_2 }
    }
}

#[derive(Debug)]
struct CapselStructive {
    val_3: u32,
    val_4: u32,
}
impl CapselStructive {
    pub fn new(val_3: u32, val_4: u32) -> CapselStructive {
        CapselStructive { val_3, val_4 }
    }
}

#[derive(Debug)]
pub struct LoadCapselStructive {
    pub cs: CapselStructive,
}
impl LoadCapselStructive {
    pub fn new(val_3: u32, val_4: u32) -> LoadCapselStructive {
        LoadCapselStructive {
            cs: CapselStructive::new(5, 6),
        }
    }
}
