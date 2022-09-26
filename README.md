# db_test

this is rusqlite test project. <br />
create project because of I use toolcahin[gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf] build this project
fail.<br />
I hope this project can help rusqlite author or me fix err.

I use Ubuntu for cross compilation, You can install the environment as follows:

1. download release/tag/toolchain/<a href="https://github.com/lifeRobot/db_test/releases/download/toolchain/gcc-linaro-5.3-20190918.tar.bz2">gcc-linaro-5.3-20190918.tar.bz2</a>(716M) to your Ubuntu dev environment /opt dir
2. run command:


    cd /opt
    tar xvjf gcc-linaro-5.3-20190918.tar.bz2 -C /
    mkdir -p /opt/cpp/linaro
    ln -s /opt/EmbedSky/gcc-linaro-5.3-2016.02-x86_64_arm-linux-gnueabihf/bin/* /opt/cpp/linaro/
3. run command test cpp dev environment:


    /opt/cpp/linaro/arm-linux-gnueabihf-gcc -v
4. git clone this project or download this project
5. if you want to change target, see
   <a href="https://github.com/lifeRobot/db_test/tree/master/.cargo/config.html">.cargo/config.toml</a>.
   <br />
   run command build this project:


    cargo build --target=armv7-unknown-linux-gnueabihf