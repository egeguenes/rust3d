use rust3d::mathlib::vector3d::Vector3d;

fn main() {
    let mut vector1 = Vector3d::new(1.0, 1.0, 1.0);
    let mut vector2 = Vector3d::new(2.0, 3.0, 4.0);

    let length1 = vector1.length();
    let squared = vector1.length_squared();
    println!("Len should be sqrt(3) and is {}", length1);
    println!("Squared should be 3 and is {}", squared);

    vector2 += vector1; // 3.0, 4.0, 5.0

    let length2 = vector2.length();
    let squared2 = vector2.length_squared();
    println!("Len should be sqrt(50) and is {}", length2);
    println!("Squared should be 50 and is {}", squared2);

}
