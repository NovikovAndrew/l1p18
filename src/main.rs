fn main() {
    let hello = "hello";
    println!("hello reverted is {}", revert(hello)); // olleh

    let unicode_word = "中国语言测试";
    println!("unicode word reverted is {}", revert(unicode_word)); // 试测言语国中
}

// Алгоритм:
// разбиваем строку на векртор char
// ревертим вектор
// вектор char собируем в строку
fn revert(s: &str) -> String {
    s.chars().rev().collect()
}