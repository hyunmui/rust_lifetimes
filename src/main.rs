use std::fmt::Display;

fn main() {
//    let r;             //           +-- 'a
//    {                       //           |
//        let x = 5;      // -+-- 'b   |
//        r = &x;             //  |        |
//    }                       // -+        |
//                            //           |
//    println!("r: {}", r);   //           |
//                            // ----------+

    // 위에 코드 수정
    let x = 5;
    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("string more longer: {}", result);

    {
        let string3 = String::from("zzz");
        let result1_3 = longest(string1.as_str(), string3.as_str());
        println!("string more longer: {}", result1_3);
    }

    // not working because lifetimes
    // let result1_4;
    // {
    //     let string4 = String::from("zzz");
    //     result1_4 = longest(string1.as_str(), string4.as_str());
    //                                             // borrowed value does not live long enough
    // }
    // println!("string more longer: {}", result1_4);
    //                                     // borrow later used here

    let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계에....");
    let first_sentence = novel.split('.')
        .next()
        .expect("문장에서 마침표'.'을 찾을 수 없습니다.");
    let i = ImportantExcerpt { part: first_sentence };

    // static lifetime
    let s: &'static str = "이 문자열은 정적 수명이다.";
}

// 같은 수명임을 나타내는 수명 제네릭
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("주목해 주세요! {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("주목하세요: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}