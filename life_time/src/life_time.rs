//生命周期与引用有效性
//Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域

//Rust 需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的

//生命周期避免了悬垂引用


{
	let r;
	{
		let x = 5;
		r = &x;
	}
	println!("r: {}", r);//被引用的对象比它的引用者存活的时间更短
}

//未初始化变量不能被使用

//借用检查器 borrow checker

//error
{
	let r; // -------+-- 'a
	// |
	{ // |
		let x = 5; // -+-----+-- 'b
		r = &x; // | |
	} // -+ |
	// |
	println!("r: {}", r); // |
	// |
	// -------+
}


//right
{
	let x = 5; // -----+-- 'b
	// |
	let r = &x; // --+--+-- 'a
	// | |
	println!("r: {}", r); // | |
	// --+ |
}

//函数中的泛型生命周期


fn main() {
	let string1 = String::from("abcd");
	let string2 = "xyz";
	let result = longest(string1.as_str(), string2);
	println!("The longest string is {}", result);
}

fn longest(x: &str, y: &str) -> &str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}


//生命周期注解语法

&i32 // a reference
&'a i32 // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

//函数签名中的生命周期注解

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		x
	} else {
		y
	}
}

fn main() {
	let string1 = String::from("long string is long");
	{
		let string2 = String::from("xyz");
		let result = longest(string1.as_str(), string2.as_str());
		println!("The longest string is {}", result);
	}
}

fn main() {
	let string1 = String::from("long string is long");
	let result;
	{
		let string2 = String::from("xyz");
		result = longest(string1.as_str(), string2.as_str());
	}
	println!("The longest string is {}", result);
}

//深入理解生命周期

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
	x
}


//结构体定义中的生命周期注解

struct ImportantExcerpt<'a> {
	part: &'a str,
}
fn main() {
	let novel = String::from("Call me Ishmael. Some years ago...");
	let first_sentence = novel.split('.')
		.next()
		.expect("Could not find a '.'");
	let i = ImportantExcerpt { part: first_sentence };
}


//生命周期省略

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}
	&s[..]
}
//1. 每一个是引用的参数都有它自己的生命周期参数。换句话说就是，有一个引用参数的函数有一个生命周期参
//数： fn foo<'a>(x: &'a i32) ，有两个引用参数的函数有两个不同的生命周期参数， fn foo<'a, 'b>(x: &'a i32,											    y: &'b i32) ，依此类推。
//2. 如果只有一个输入生命周期参数，那么它被赋给所有输出生命周期参数： fn foo<'a>(x: &'a i32) -> &'a i32 。
//3. 如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self ，那么 self 的生
//命周期被赋给所有输出生命周期参数。这使得方法写起来更简洁

//方法定义中的生命周期注解
//声明和使用生命周期参数的位置依赖于生命周期参数是否同结构体字段或方法参数和返回值相关

# struct ImportantExcerpt<'a> {
	# part: &'a str,
	# }
#
impl<'a> ImportantExcerpt<'a> {
	fn level(&self) -> i32 {
		3
	}
}



# struct ImportantExcerpt<'a> {
	# part: &'a str,
	# }
#
impl<'a> ImportantExcerpt<'a> {
	fn announce_and_return_part(&self, announcement: &str) -> &str {
		println!("Attention please: {}", announcement);
		self.part
	}
}

//静态生命周期

let s: &'static str = "I have a static lifetime.";

//结合泛型类型参数、trait bounds 和生命周期

use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display
{
	println!("Announcement! {}", ann);
	if x.len() > y.len() {
		x
	} else {
		y
	}
}




