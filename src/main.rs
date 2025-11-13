fn main() {
    let out = b"Hello from Rust!";
    let width: u32 = 24;
    let size = width.checked_sub(out.len().try_into().unwrap()).unwrap_or(0);

    let sample = width.checked_sub(25).unwrap_or(0);
    println!("Sample width after subtraction: {}", sample);


    let nums = [1,2,3,4,5];
    let str = format!("{:?}", nums);
    let str24 = width.to_string();
    
    for(i, &nums) in nums.iter().enumerate(){
        println!("nums:{}", nums);
        println!("Width: {}", width);
        println!("{}", String::from_utf8_lossy(out));
        
    }
    println!("Array as string: {:?}", str);
    println!("Sub width with out: {}", size);
    println!("{}", 100);
    println!("Stringify width u32 to String: {}", str24);

}