//为了创建一个新的，空的 vector，可以调用 Vec::new 函数：
//vector 是同质的（homogeneous）

let v: Vec<i32>  = Vec::new();

//类型注解不是必须的，宏
let v = vec![1,2,3];

// 更新
v.push(1);

//丢弃 vector 时也会丢弃其所有元素
{
 let v2 = vec::new();
}

//read

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);

// panic! or  None 
// 无效引用


//使用枚举还存储多种类型

enum Sp {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    Sp::Int(32),
    Sp::Text(String::from("blue")),
    Sp::Float(10.21),
]































