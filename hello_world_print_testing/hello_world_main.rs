fn main ()
{
    let some_random_int: i128 = 5561;
    let binary_input_test : i8 = 0b0110;
    let hex_input_test: i8 = 0xA;
    let floating_point_value: f64 = 365.25;
    let some_random_string = "stored text input";
    print!("test text \n");
    print!("some_number output {}\n",some_random_int);
    print!("binary input test binary: {:b} int:{}\n",binary_input_test,binary_input_test);
    print!("hex input test hex:{:x} binary:{:b} int:{} \n",hex_input_test,hex_input_test,hex_input_test);
    print!("floating value output test float: {:.3}\n",floating_point_value); // seem the .# is the # of digits after the decimal desired
    print!("output string variable: {some_random_string}\n");
    print!("testing other input formats {hex_input_test}\n");

    eprint!("test error text\n");// must test more didnt do anything different than normal



}
