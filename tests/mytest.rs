
#[test]
pub fn demo01(){
    let word = "hello";
    println!("{:?}", get_first_char(word)); // 输出: Some('h')

    fn get_first_char(s: &str) -> Option<char> {
        let first = s.chars().next()?; // `?` 解包 Some(c)，返回 `c`
        Some(first)
    }}