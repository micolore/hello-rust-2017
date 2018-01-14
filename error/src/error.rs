//Rust 将错误组合成两个主要类别：可恢复错误（recoverable）和 不可恢复错误（unrecoverable）
//可恢复错误
//通常代表向用户报告错误和重试操作是合理的情况，比如未找到文件。
//不可恢复错误通常是 bug 的同义词，比如尝试访问超过数组结尾的位置。

//panic! 与不可恢复的错误
fn main() {
	panic!("crash and burn");
}

//使用 panic! 的 backtrace

fn main() {
	let v = vec![1, 2, 3];
	v[100];
}

//RUST_BACKTRACE=1 cargo run


//Result 与可恢复的错误

use std::fs::File;
fn main() {
	let f = File::open("hello.txt");
}


let f: u32 = File::open("hello.txt");

use std::fs::File;
fn main() {
	let f = File::open("hello.txt");
	let f = match f {
		Ok(file) => file,
		Err(error) => {
			panic!("There was a problem opening the file: {:?}", error)
		},
	};
}



//匹配不同的错误

use std::fs::File;
use std::io::ErrorKind;
fn main() {
	let f = File::open("hello.txt");
	let f = match f {
		Ok(file) => file,
		Err(ref error) if error.kind() == ErrorKind::NotFound => {
				 match File::create("hello.txt") {
					 Ok(fc) => fc,
					 Err(e) => {
						 panic!("Tried to create file but there was a problem: {:?}",e)
					 },
				 }
		},
		Err(error) => {
			panic!("There was a problem opening the file: {:?}",error)
		},
	};
}
//在模式的上下文中， & 匹配一个引用并返回它的值，而ref 匹配一个值并返回一个引用


//失败时 panic 的捷径： unwrap 和 expect

use std::fs::File;
fn main() {
	let f = File::open("hello.txt").unwrap();
}

use std::fs::File;
fn main() {
	let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

//传播错误


use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
	let f = File::open("hello.txt");
	let mut f = match f {
		Ok(file) => file,
		Err(e) => return Err(e),
	};
	let mut s = String::new();
	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(e) => Err(e),
	}
}

//传播错误的捷径： ?

use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
	let mut f = File::open("hello.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}


use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
	let mut s = String::new();
	File::open("hello.txt")?.read_to_string(&mut s)?;
	Ok(s)
}



//? 只能被用于返回 Result 的函数

use std::fs::File;
fn main() {
	let f = File::open("hello.txt")?;
}




//panic! 还是不 panic!
//如果代码 panic，就没有恢复的可能。你可以选择对任何错误场景都调用 panic! 
//选择返回 Result 值的话，就将选择权交给了调用者，而不是代替他们做出决定

//示例、代码原型和测试：非常适合 panic


//当你比编译器知道更多的情况


//错误处理指导原则
//有害状态并不包含 预期 会偶尔发生的错误
//之后的代码的运行依赖于不再处于这种有害状态
//当没有可行的手段来将有害状态信息编码进所使用的类型中的情况

//创建自定义类型作为验证

loop {
	// snip
	let guess: i32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
	};
	if guess < 1 || guess > 100 {
		println!("The secret number will be between 1 and 100.");
		continue;
	}
	match guess.cmp(&secret_number) {
		// snip
	}


pub struct Guess {
	value: u32,
}
impl Guess {
	pub fn new(value: u32) -> Guess {
		if value < 1 || value > 100 {
			panic!("Guess value must be between 1 and 100, got {}.", value);
		}
		Guess {
			value
		}
	}
	pub fn value(&self) -> u32 {
		self.value
	}
}













