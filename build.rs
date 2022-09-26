// 在linaro E9V3中会编译失败，如果想在E9V3中使用sqlite3，需要使用C/CPP然后用Rust进行ffi来调用使用<br />
// 目前暂时使用sled，使用C++来搞还有点问题
pub fn main() {
    // println!("-L /opt/EmbedSky/gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf/rootfs/usr/lib");
}