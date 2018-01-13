pub  mod a {
     pub mod series	{
	    pub mod of {
	        pub fn nested_modules() {
		}	
	    }
     }
}

enum TrafficLight {
	Red,
	Yellow,
	Green,
}

use TrafficLight::{Red, Yellow};
//use TrafficLight::*; xx命名空间的所有项：
use a::series::of;//忽略模块直接使用函数

fn main() {
    //a::series::of::nested_modules();
    of::nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green();
}
