// Patterns That Bind to Values

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopack = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // let home_adv = IpAddrAdv::V4(123, 1, 2, 3);
    // let loopack_adv = IpAddrAdv::V6(String::from("::1"));

    // let m = Message::Write(String::from("hello"));
    // m.call();

    // let some_number = Some(5);
    // let some_char = Some('a');

    // let absent_number: Option<u8> = None;
    // let coin = Coin::Nickel;
    // let coin_alaska = CoinAdv::Quarter(UsState::Alaska);
    // values_in_coin(coin_alaska);

    let config_max = Some(2u8);
    match  config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }


}

fn values_in_coin(coin: CoinAdv) -> u8 {
    match coin {
        CoinAdv::Penny => 1,
        CoinAdv::Nickel => 5,
        CoinAdv::Dime => 10,
        CoinAdv::Quarter(us_state) => {
            println!("State:: {us_state:?}");
            25
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum CoinAdv {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Message {
    Quit,
    Write(String),
    Move { x: u32, y: u32 },
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //
    }
}

enum IpAddrAdv {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
