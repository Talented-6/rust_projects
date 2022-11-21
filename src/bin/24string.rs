fn main() {
    // chars() and bytes()
    let s = "中国人";
    for c in s.chars() {
        println!("{}", c);
    }
    println!();
    for b in s.bytes() {
        println!("{}", b);
    }
    println!();
    // String can not be indexed, but can be sliced.
    println!("{}", &s[0..3]);
    // push
    let mut s = "中国人".to_string();
    println!("{}", &s[0..3]);
    s.push('的');
    s.push_str("未来");
    println!("{}", s);
    // insert
    s.insert_str(9, "拥有美好");
    println!("{}", s);
    // replace
    // relpacen
    // replace_range
    s = s.replace("中国人", "所有人");
    println!("{}", s);
    // pop
    // remove
    // truncate
    // clear
    s.pop();
    dbg!(&s);
    s.remove(0);    // only need to input the start index of a char
    dbg!(&s);
    s.truncate(6);
    dbg!(&s);
    s.clear();
    dbg!(&s);
    // escape characters
    println!("I am writing \x52\x75\x73\x74!");
    println!("\u{211D}");
    println!("A
   B
            C\
                            D");
    println!(r###"A string with "# in it. And even "##!"###); // perhaps because there is 5
    // unsupported chars in it, so it needs to have 3 pairs of `#`
}
