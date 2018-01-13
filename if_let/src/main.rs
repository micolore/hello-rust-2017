//if let 控制流

let some_u8_value = Some(0u8);
match some_u8_value {
	Some(3) => println!("three"),
	_ => (),
}

//如果只关心==3的情况下应该怎么做呢

if let Some(3) == some_u8_value {
  	println!("three");
}

// match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍

// if let else


let mut count = 0;
if let Coin::Quarter(state) = coin {
	println!("state quarter form {:?}!",state);
} else {
	count += 1;
}

