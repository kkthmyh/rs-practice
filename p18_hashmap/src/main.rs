use std::collections::HashMap;

fn  main() {
    //  创建新的hashmap
    let mut m1 = HashMap::new();
    m1.insert("rust".to_string(), 100);
    println!("Hello, world!");


    // HashMap所有权机制
    // 对于实现了Copy trait的类型如i32，值会被复制到HashMap中不会丢失所有权
    // 对于拥有所有权的值，如String，值会被移动，所有权将转移给HashMap
    // 如果将值的引用插入到HashMap中，值本身不会移动
    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let mut m2 = HashMap::new();
    // m2.insert(s1, s2);
    // println!("s1 is {},s2 is {}", s1 , s2); // 编译不通过
    
     m2.insert(&s1, &s2);
    println!("s1 is {},s2 is {}", s1 , s2); // 可以编译


    // 访问HashMap中的值
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);

    let blue_score = scores.get(&String::from("blue"));

    match blue_score {
        None => println!("Get None"),
        Some(i) => println!("Get score is {}" , i),
    }


    // 遍历HashMap
    for (k,v) in &scores {
        println!("k is {},v is {}" , k , v);
    }


    // 更新HashMap
    
    // 1、直接替换现有的v
    let mut map1 = HashMap::new();
    map1.insert(String::from("rust"), 100);
    map1.insert(String::from("rust"), 999);
    println!("map1 is {:?}" , map1);

    // 2、只在k不存在的情况下才插入v
    let mut map2 = HashMap::new();
    map2.entry(String::from("rust")).or_insert(100);
    map2.entry(String::from("rust")).or_insert(999);
    println!("map2 is {:?}" , map2);

    // 3、基于现有v来更新v
    let text = String::from("hello world hello rust");
    let mut wmap = HashMap::new();

    for item in text.split_whitespace() {
        let count = wmap.entry(item).or_insert(0);
        *count += 1
    }

    println!("wmap is {:?}", wmap);
}
