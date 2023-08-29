fn main() {
    // for x in 1..8{
    //     for y in  (8-x)..=1{
    //         print!("+");
    //     }
    //     // for y in  1..8{
    //     //     print!("*");
    //     // }
    //     // println!();
    //     // println!("*");
    // }

    let mut i = 1;
    let mut j = 1;
    let mut y = 0;


    while i != 8 {
        j = 8-i;
        while j != 0 {
            print!(" ");
            if j == 0 {
               
                break;
            }

            j -= 1;
        }

        y = y+1;


        for k in 1..(i+y){
            print!("*");
        }

        if i == 8 {
            break;
        }
        println!();

        i = i + 1;
    }
        
    
}