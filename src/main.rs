fn main() {
    println!("Hello, world!");

    let mut i = 0u32;

    let ret = loop {
    
        println!("{}", i);

        if i > 50 {
        
            break i%12;
        }

        i += match i%9 {
        
            0       => 1,
            1..=3    => 2,
            4|6|8   => 3,
            _       => 4
        
        }
    };

    println!("Final value: {}", ret);

    let ys: [i32; 500] = [5; 500];

    println!("{:?} {:?}", ys[0], ys[1]);
}
