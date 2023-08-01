// 一个简单的声明宏，实现支持类似python的列表表达式语法
macro_rules! list_comprehension {
    // 匹配基本情况，没有筛选条件的推导表达式
    ([$expression:expr; for $iterator:ident in $iterable:expr]) => {{
        let mut result = vec![];
        for $iterator in $iterable {
            result.push($expression);
        }
        result
    }};

    // 匹配有筛选条件的推导表达式
    ([$expression:expr; for $iterator:ident in $iterable:expr, if $condition:expr]) => {{
        let mut result = vec![];
        for &$iterator in $iterable {
            if $condition {
                result.push($expression);
            }
        }
        result
    }};
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // 没有筛选条件的类似python的列表表达式：将列表中每个数字乘以 2，并生成一个新的列表
    let result1 = list_comprehension!([x * 2; for x in &numbers]);
    println!("Result 1: {:?}", result1);  // 输出: [2, 4, 6, 8, 10]
    
    // 带有筛选条件的类似python的列表表达式：将列表中大于 2 的数字乘以 2，并生成一个新的列表
    let result2 = list_comprehension!([x * 2; for x in &numbers, if x > 2]);
    println!("Result 2: {:?}", result2);  // 输出: [6, 8, 10]
}

// 上述代码在Rust编译期进行了宏展开，生成以下对应代码。
/*
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
// 一个简单的声明宏，实现支持类似python的列表表达式语法
macro_rules! list_comprehension {
    ([$expression : expr ; for $iterator : ident in $iterable : expr]) =>
    {
        {
            let mut result = vec! [] ; for $iterator in $iterable
            {
                result.push($expression) ;
                // 匹配基本情况，没有筛选条件的推导表达式
            } result
        }
    } ;
    ([$expression : expr ; for $iterator : ident in $iterable : expr, if
    $condition : expr]) =>
    {
        {
            let mut result = vec! [] ; for & $iterator in $iterable
            {
                if $condition
                {
                    result.push($expression) ;

                    // 匹配有筛选条件的推导表达式
                }
            } result
        }
    } ;
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // 没有筛选条件的类似python的列表表达式：将列表中每个数字乘以 2，并生成一个新的列表
    let result1 =
        {
            let mut result =  // 输出: [2, 4, 6, 8, 10]

                // 带有筛选条件的类似python的列表表达式：将列表中大于 2 的数字乘以 2，并生成一个新的列表
                // 输出: [6, 8, 10]
                ::alloc::vec::Vec::new();
            for x in &numbers { result.push(x * 2); }
            result
        };
    { ::std::io::_print(format_args!("Result 1: {0:?}\n", result1)); };
    let result2 =
        {
            let mut result = ::alloc::vec::Vec::new();
            for &x in &numbers { if x > 2 { result.push(x * 2); } }
            result
        };
    { ::std::io::_print(format_args!("Result 2: {0:?}\n", result2)); };
}
*/