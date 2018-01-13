//netwok 模块
pub mod client; //client 模块声明 ==  contents of client.rs
mod network;


//模块文件系统的规则
//如果一个叫做 foo 的模块没有子模块，应该将 foo 的声明放入叫做 foo.rs 的文件中。
//如果一个叫做 foo 的模块有子模块，应该将 foo 的声明放入叫做 foo/mod.rs 的文件中

//私有性规则 类似 java private
//1. 如果一个项是公有的，它能被任何父模块访问
//2. 如果一个项是私有的，它能被其直接父模块及其任何子模块访问


//super 访问父模块

