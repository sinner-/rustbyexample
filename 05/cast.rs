fn main() {
    let decimal = 65.4321_f32;

    let integer = decimal as u8;
    let character = integer as char;

    println!("casting: {} -> {} -> {}", decimal, integer, character);

    //when casting any value to an unsigned type, T,
    //std::T::Max + 1 is added or subtracted until the value
    //fits into the new type

    //1000 already fits into a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    //1000 - 256 - 256 - 256 = 232
    //under the hood, the first 8 least significant bits (LSB) are kept,
    //while the rest towards to the MSB gets truncated.
    println!("1000 as u8 is: {}", 1000 as u8);
    //-1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    println!("1000 mod 256 is: {}", 1000 % 256);

    //When casting to a signed type, the (bitwise) result is the same as
    //first casting to the corresponding unsigned type. If the most significant
    //bit of that value is 1 then the value is negative

    //unless it already fits
    println!("128 as a i16 is: {}", 128 as i16);
    //128 as u8 -> 128 who's two complement in eight bits is:
    println!("128 as a i8 is: {}", 128 as i8);

    //repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as u8 is: {}", 1000 as u8);
    println!("232 as a i8 is: {}", 232 as i8);
}
