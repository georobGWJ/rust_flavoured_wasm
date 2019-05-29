// Foreign Function Interface (ffi); a means that programming languages can
// share and use each others libraries.

// This SDK is based on the Intrinsic Function Library defined by the Crobots project.
extern "C" {
    pub fn scan(angle:i32, resolution: i32) -> i32;
    pub fn cannon(angle: i32, range: i32) -> i32;
    pub fn drive(angle: i32, speed: i32) -> i32;
    pub fn damage() -> i32;
    pub fn speed() -> i32;
    pub fn loc_x() -> i32;
    pub fn loc_y() -> i32;
    pub fn rand(limit: i32) -> i32;
    pub fn wsqrt(number: i32) -> i32;
    pub fn wsin(degree: i32) -> i32;
    pub fn wcos(degree: i32) -> i32;
    pub fn wtan(degree: i32) -> i32;
    pub fn watan(degree: i32) -> i32;
    pub fn plot_course(tx: i32, ty: i32) -> i32;
}

pub fn scan(angle:i32, resolution: i32) -> i32{
    unsafe { ffi::scan(angle, resolution) }
}

pub fn cannon(angle: i32, range: i32) -> i32{
    unsafe { ffi::(angle, range) }
}

pub fn drive(angle: i32, speed: i32) -> i32{
    unsafe { ffi::(angle, speed) }
}

pub fn damage() -> i32{
    unsafe { ffi::damage() }
}

pub fn speed() -> i32{
    unsafe { ffi::speed() }
}

pub fn loc_x() -> i32{
    unsafe { ffi::loc_x() }
}

pub fn loc_y() -> i32{
    unsafe { ffi::loc_y() }
}

pub fn rand(limit: i32) -> i32{
    unsafe { ffi::rand(limit) }
}

pub fn wsqrt(number: i32) -> i32{
    unsafe { ffi::wsqrt(number) }
}

pub fn wsin(degree: i32) -> i32{
    unsafe { ffi::wsin(degree) }
}

pub fn wcos(degree: i32) -> i32{
    unsafe { ffi::wcos(degree) }
}

pub fn wtan(degree: i32) -> i32{
    unsafe { ffi::wtan(degree) }
}

pub fn watan(degree: i32) -> i32{
    unsafe { ffi::watan(degree) }
}

pub fn plot_course(tx: i32, ty: i32) -> i32{
    unsafe { ffi::plot_course(tx, ty) }
}

