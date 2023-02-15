use std::io;

struct Product {
    name: String,
    price: f32,
}

fn main() {
    let products = vec![
        Product {
            name: String::from("Coke"),
            price: 1.0,
        },
        Product {
            name: String::from("Snickers"),
            price: 1.5,
        },
        Product {
            name: String::from("Chips"),
            price: 1.25,
        },
    ];

    println!("Welcome to the vending machine!\n");

    for (index, product) in products.iter().enumerate() {
        println!("{}. {} - ${}", index + 1, product.name, product.price);
    }

    println!("\nEnter the number of the product you want: ");

    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read input.");

    let selection: usize = selection.trim().parse().expect("Invalid input.");
    let product = &products[selection - 1];

    println!(
        "\nYou have selected {}. The price is ${}.\n",
        product.name, product.price
    );

    let mut payment = String::new();
    println!("Please enter your payment: ");

    io::stdin()
        .read_line(&mut payment)
        .expect("Failed to read input.");

    let payment: f32 = payment.trim().parse().expect("Invalid input.");

    if payment < product.price {
        println!("\nError: Insufficient payment. Please try again.");
    } else {
        let change = payment - product.price;
        println!("\nProduct dispensed. Your change is ${:.2}.", change);
    }

    println!("\nThank you for using the vending machine!");
}
