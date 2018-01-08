//函数 area 本应该计算一个长方形的面积，不过函数却有两个参数。这两个参数是相关联的，不过程序自身却哪里
//也没有表现出这一点。将长度和宽度组合在一起将更易懂也更易处理
fn main() {
	let rect1 = (50, 30);

        println!(
	    "The area of the rectangle is {} square pixels.",
	    area(rect1)
        );
}

 fn area(dimensions: (u32, u32)) -> u32 {
       dimensions.0 * dimensions.1
}
//元组帮助我们增加了一些结构性，现在在调用 area 的时候只用传递一个参数。不
//过在另一方面这个方法却更不明确了：元组并没有给出它元素的名称，所以计算变得更费解了，因为不得不使用索
//引来获取元组的每一部分：

//使用结构体重构：增加更多意义

struct Rectangle {
	length: u32,
	width: u32,
}
fn main() {
	let rect1 = Rectangle { length: 50, width: 30 };
	println!(
		"The area of the rectangle is {} square pixels.",
		area(&rect1)
		);
}
fn area(rectangle: &Rectangle) -> u32 {
		rectangle.length * rectangle.width
}

//通过衍生 trait 增加实用功能


#[derive(Debug)]
struct Rectangle {
	length: u32,
	width: u32,
}
fn main() {
	let rect1 = Rectangle { length: 50, width: 30 };
	
	println!("rect1 is {:?}", rect1);
}
//Rust 为我们提供了很多可以通过 derive 注解来使用的 trait，他们可以为我们的自定义类型增加实用的行为。

//方法语法

//方法 与函数类似：他们使用 fn 关键和名字声明，可以拥有参数和返回值，同时包含一些代码会在某处被调用时
//执行。不过方法与函数是不同的，因为他们在结构体（或者枚举或者 trait 对象，将分别在第六章和第十七章讲
//解）的上下文中被定义，并且他们第一个参数总是 self ，它代表方法被调用的结构体的实例

#[derive(Debug)]
struct Rectangle {
	length: u32,
	width: u32,
}
impl Rectangle {
	fn area(&self) -> u32 {
	   self.length * self.width
}
}
fn main() {
	let rect1 = Rectangle { length: 50, width: 30 };
	println!(
	  "The area of the rectangle is {} square pixels.",
	   rect1.area()
	);
}

//为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（ impl 是 implementation 的缩写）。接着
//将函数移动到 impl 大括号中，并将签名中的第一个（在这里也是唯一一个）参数和函数体中其他地方的对应参数
//改成 self 。然后在 main 中将我们调用 area 方法并传递 rect1 作为参数的地方，改成使用 方法语法（method
//syntax）在 Rectangle 实例上调用 area 方法。方法语法获取一个实例并加上一个点号后跟方法名、括号以及任何参数。


# #[derive(Debug,Copy,Clone)]
# struct Point {
# x: f64,
# y: f64,
# }
#
# impl Point {
# fn distance(&self, other: &Point) -> f64 {
# let x_squared = f64::powi(other.x - self.x, 2);
# let y_squared = f64::powi(other.y - self.y, 2);
#
# f64::sqrt(x_squared + y_squared)
# }
# }
# let p1 = Point { x: 0.0, y: 0.0 };
# let p2 = Point { x: 5.0, y: 6.5 };
p1.distance(&p2);
(&p1).distance(&p2);





//带有更多参数的方法


fn main() {
	let rect1 = Rectangle { length: 50, width: 30 };
	let rect2 = Rectangle { length: 40, width: 10 };
	let rect3 = Rectangle { length: 45, width: 60 };
	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

//通过观察调用位置的代码可以看出参数是什么类型的： rect1.can_hold(&rect2)
//传入了 &rect2 ，它是一个 Rectangle 的实例 rect2 的不可变借用。这是可以理解的，因为我们只需要读取
//rect2 （而不是写入，这意味着我们需要一个可变借用）而且希望 main 保持 rect2 的所有权这样就可以在调用
//这个方法后继续使用它

# #[derive(Debug)]
# struct Rectangle {
# length: u32,
# width: u32,
# }
#
impl Rectangle {
	fn area(&self) -> u32 {
		self.length * self.width
	} 
	fn can_hold(&self, other: &Rectangle) -> bool {
	 	self.length > other.length && self.width > other.width
 	}
}

//关联函数 String::from

# #[derive(Debug)]
# struct Rectangle {
# length: u32,
# width: u32,
# }
#
impl Rectangle {
	fn square(size: u32) -> Rectangle {
		Rectangle { length: size, width: size }
	}
}

//使用结构体名和 :: 语法来调用这个关联函数：比如 let sq = Rectangle::square(3); 。这个方法位于结构体的命名
//空间中： :: 语法用于关联函数和模块创建的命名空间


//多个 impl 块

# #[derive(Debug)]
# struct Rectangle {
# length: u32,
# width: u32,
# }
#
impl Rectangle {
	fn area(&self) -> u32 {
	   self.length * self.width
        }
}
impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
	   self.length > other.length && self.width > other.width
       }
}


//结构体让我们可以在自己的范围内创建有意义的自定义类型。通过结构体，我们可以将相关联的数据片段联系起来
//并命名他们来使得代码更清晰。方法允许为结构体实例指定行为，而关联函数将特定功能置于结构体的命名空间中
//并且无需一个实例。结构体并不是创建自定义类型的唯一方法









