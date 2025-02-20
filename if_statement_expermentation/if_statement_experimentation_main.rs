fn main ()
{
let condition_1: i8 = 3;
let condition_2:i8 = 5;

    for count in 1..32
    {
        if (count % condition_1 == 0) && (count % condition_2 == 0)
        {
            print!("fizzbuzz\n");
            continue;
        }
        if  count % condition_1 == 0
        {
            print!("fizz");
        }
        else if count % condition_2 == 0
        {
            print!("buzz");
        }
        else
        {
            print!("{count}");
        }
        print!("\n")
    }
}
