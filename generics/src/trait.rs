//定义共享的行为  类似javaii的

//定义 trait
//一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的
//行为了。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。

pub trait Summarizable {
	fn summary(&self) -> String;
}

