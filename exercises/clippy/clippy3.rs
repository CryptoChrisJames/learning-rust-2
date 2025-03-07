// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    my_option.unwrap();

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    println!("This Vec is empty, see? {:?}", vec![1, 2, 3, 4, 5].resize(0, 5));

    let mut value_a = 45;
    let mut value_b = 66;
    let mut temp = 0;
    // Let's swap these two!
    temp = value_a;
    value_a = value_b;
    value_b = temp;
    println!("value a: {}; value b: {}", value_a, value_b);
}
