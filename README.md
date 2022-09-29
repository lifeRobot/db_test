# describe

this is rusqlite test project. <br />
create project because of I use toolcahin[gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf] build this project
fail.<br />
I hope this project can help rusqlite author or me fix err.

# build fail describe
1. I try build project by all linaro/arm-linux-gnueabihf version from <a href="https://releases.linaro.org/components/toolchain/binaries/">linaro/arm-linux-gnueabihf</a> webside. any linaro/arm-linux-gnueabihf will be build fail, the build err msg always 'Dwarf Error: found dwarf version '5', this reader only handles version 2, 3 and 4 information.' and 'undefined reference to `fcntl64'', see <a href="https://github.com/lifeRobot/db_test/blob/master/build_log/build_err.txt">build_log/build_err.txt</a>
2. I think the error may be caused by the gcc version, but I use any <a href="https://releases.linaro.org/components/toolchain/binaries/">linaro/arm-linux-gnueabihf</a> build <a herf="https://github.com/sqlite/sqlite">sqlite</a>, and then will be build success
3. now, I think the error caused by the <a href="https://github.com/rusqlite/rusqlite/blob/master/libsqlite3-sys/build.rs">libsqlite3-sys/build.rs</a>.

# build project
I use Ubuntu for cross compilation, You can install the environment as follows:
1. download release/tag/toolchain/<a href="https://github.com/lifeRobot/db_test/releases/download/toolchain/gcc-linaro-5.3-20190918.tar.bz2">gcc-linaro-5.3-20190918.tar.bz2</a>(716M) to your Ubuntu dev environment /opt dir
2. run command:
``` 
cd /opt
tar xvjf gcc-linaro-5.3-20190918.tar.bz2 -C /
mkdir -p /opt/cpp/linaro
ln -s /opt/EmbedSky/gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf/bin/* /opt/cpp/linaro/
```
3. run command test cpp dev environment:
```
/opt/cpp/linaro/arm-linux-gnueabihf-gcc -v
```
4. git clone this project or download this project
5. if you want to change target, see
   <a href="https://github.com/lifeRobot/db_test/blob/master/.cargo/config.toml">.cargo/config.toml</a>.
   <br />
   run command build this project:
```
cargo build --target=armv7-unknown-linux-gnueabihf
```
