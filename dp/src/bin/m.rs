#[rustfmt::skip]
mod fast_input {
    #[macro_export]macro_rules! input{($($r:tt)*)=>{let s={use std::io::Read;let mut s=String::new();std::io::stdin().read_to_string(&mut s).unwrap();let input=Box::leak(s.into_boxed_str());input};let mut iter=s.split_ascii_whitespace();input_inner!{iter,$($r)*}};}
    #[macro_export]macro_rules! input_inner{($iter:expr)=>{};($iter:expr,)=>{};($iter:expr,$var:ident:$t:tt$($r:tt)*)=>{let $var=read_value!($iter,$t);input_inner!{$iter $($r)*}};}
    #[macro_export]macro_rules! read_value{($iter:expr,($($t:tt),*))=>{($(read_value!($iter,$t)),*)};($iter:expr,[$t:tt;$len:expr])=>{(0..$len).map(|_|read_value!($iter,$t)).collect::<Vec<_>>()};($iter:expr,Chars)=>{read_value!($iter,String).chars().collect::<Vec<char>>()};($iter:expr,Usize1)=>{read_value!($iter,usize)-1};($iter:expr,$t:ty)=>{$iter.next().unwrap().parse::<$t>().expect("Parse error")};}
}
fn main() {
    input!(n: usize, text: [Chars; n]);

    let mut ans = vec![];
    for v in text {
        if v.len() == 1 {
            ans.push(0);
            continue;
        }
        let mut vv = v.clone();
        let mut index = 1;
        let mut count = 0;
        loop {
            if vv[index - 1] != '1' && vv[index - 1] == vv[index] {
                vv[index] = '1';
                count += 1;
            }
            if index == v.len() - 1 {
                break;
            }
            if vv[index - 1] != '1' && vv[index - 1] == vv[index + 1] {
                vv[index + 1] = '1';
                count += 1;
            }

            index += 1;
        }
        ans.push(count);
    }
    ans.iter().for_each(|x| println!("{}", x));
}
