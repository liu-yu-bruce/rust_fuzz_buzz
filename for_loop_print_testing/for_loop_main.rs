fn main ()
{
    let lower_bound:i8 = 0;
    let upper_bound:i8 = 16;

    for count in lower_bound..upper_bound
    {
        print!("{}\n",count);
    }
    for count in 0..16
    {
        for dicount in 0..6
        {
            print!("{count:x}, {dicount:b}\n");
        }
    }
}
