// asciiart.rs

fn main() {
    for y in -12..13 {
        for x in -39..40 {
            let ca = x as f32 * 0.0458;
            let cb = y as f32 * 0.08333;
            let mut a:f32 = ca;
            let mut b:f32 = cb;
            let mut flag:bool = true;
            for i in 0..16 {
                let t:f32 = a*a-b*b+ca;
                b = 2.0*a*b+cb;
                a = t;
                if (a*a+b*b) > 4.0 {
                    print!("{:X}",i);
                    flag = false;
                    break;
                }
            }
            if flag {
                print!(" ");
            }
        }
        println!(" ");
    }
}
