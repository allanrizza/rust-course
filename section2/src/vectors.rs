pub fn vectors() {
    let mut nums = vec![1,2,3];

    nums.push(4);

    println!("{:?}", nums);

    nums.pop();

    println!("{:?}", nums);

    let mut vec = Vec::new(); //vec!
    vec.push("Test");
    vec.push("String");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);

    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    println!("{}", v.len());
}