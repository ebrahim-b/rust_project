fn main() {
    let _x:i32 = 12;
    let c:char = 'ف';
    println!("{}",c as i32);

    let name:&str = "Ebrahim";
    println!("{}",name);

    let family:String = "Barati".to_string();
    let full_name = name.to_string() + " " +  &family;

    let full_name_2 = format!("{} {}",name,family);

    println!("{}",full_name);
    println!("{}",full_name_2);

    let arr:[i32;3] = [2,95,63];
    println!("{:?}",arr);

    let arr_2 = [5;5];
    println!("{:?}",arr_2);
}
