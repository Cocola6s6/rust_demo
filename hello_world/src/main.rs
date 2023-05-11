fn main() {
    // 1、基本类型的自动拷贝
    let x = 5;
    let y = x;

    // 2、复合类型的移动
    let s1 = String::from("hello");
    let s2 = s1;

    // 3、复合类型的深拷贝
    let str1 = String::from("hello");
    let str2 = str1.clone();

    println!("x = {}, y = {}", x, y);
    println!("s1 = {}, s2 = {}", s1, s2);
    println!("str1 = {}, str2 = {}", str1, str2);
}
