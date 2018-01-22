//Rust 的核心功能（之一）是 所有权（ownership)

//内存被一个所有权系统管理，它拥有一系列的规则使编译器在编译时进行检查
//任何所有权系统的功能都不会导致运行时开销

//stack  栈以放入值的顺序存储并以相反顺序取出值。这也被称作 后进先出（last in, first out） 盘子
//存储指针 已知固定大小


//heap 缺乏组织 分配内存，必须通过指针进行访问


//所有权规则
//1、每一个值都被它 的所有者变量拥有
//2、值在任意时刻只能被一个所有者拥有
//3、当所有者离开作用域，这个值将被丢弃

//变量作用域

//String  与 字符串字面值的区别

//内存与分配

//硬编码


//变量与数据交互的方式（一）移动
let s1 = String::from("hello");
let s2 = s1;

//double free
//浅拷贝&深拷贝
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
//上面是移动而不是浅拷贝 s1 无效了

//变量与数据交互的方式（二）克隆

let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);

//只在栈上的数据：拷贝

let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);

//如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用

//所有整数类型，比如 u32 。
//布尔类型， bool ，它的值是 true 和 false 。
//所有浮点数类型，比如 f64 。
//元组，当且仅当其包含的类型也都是 Copy 的时候。 (i32, i32) 是 Copy 的，不过 (i32, String) 就不是。


//所有权和函数
fn main() {
	let s = String::from("hello"); // s comes into scope.
	takes_ownership(s); // s's value moves into the function...

	// ... and so is no longer valid here.
	let x = 5; // x comes into scope.
	makes_copy(x); // x would move into the function,
	// but i32 is Copy, so it’s okay to still
	// use x afterward.
} // Here, x goes out of scope, then s. But since s's value was moved, nothing
// special happens.
fn takes_ownership(some_string: String) { // some_string comes into scope.
	println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope.
	println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


//返回值和作用域

fn main() {
	let s1 = gives_ownership(); // gives_ownership moves its return
	// value into s1.
	let s2 = String::from("hello"); // s2 comes into scope.
	let s3 = takes_and_gives_back(s2); // s2 is moved into
	// takes_and_gives_back, which also
	// moves its return value into s3.
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.
fn gives_ownership() -> String { // gives_ownership will move its
	// return value into the function
	// that calls it.
	let some_string = String::from("hello"); // some_string comes into scope.
	some_string // some_string is returned and
		// moves out to the calling
		// function.
}
// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
	// scope.
	a_string // a_string is returned and moves out to the calling function.
}

//变量的所有权总是遵循相同的模式：将值赋值给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其
//值将通过 drop 被清理掉，除非数据被移动为另一个变量所有
fn main() {
	let s1 = String::from("hello");
	let (s2, len) = calculate_length(s1);
	println!("The length of '{}' is {}.", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
	let length = s.len(); // len() returns the length of a String.
	(s, length)
}

//引用（references）











