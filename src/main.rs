// enum IpAddreKind {
//   V4(u8, u8, u8, u8),
//   V6(String)
// }

// #[derive(Debug)]
// enum IpAddr {
//   V4(u8, u8, u8, u8),
//   V6(String)
// }

// fn main() {
//   // let four = IpAddreKind::V4;
//   // let six = IpAddreKind::V6;

//   // route(four);
//   // route(six);

//   let home = IpAddr::V4(127, 0, 0, 1);

//   let loopback = IpAddr::V6(String::from("::1"));
//   println!("home: {:?}", home);
//   println!("loopback: {:?}", loopback);

// }

// fn route(ip_kind: IpAddreKind) {

// }

// enum Message {
//   Quit,
//   Move { x: i32, y: i32 },
//   Write(String),
//   ChangeColor(i32, i32, i32),
// }

// impl Message {
//   fn call(&self) {
//     println!("Calling",)
//   }
// }

// fn main() {
//   let m =  Message::Write(String::from("Hello"));
//   m.call();
// }

enum Option<T> {
  None,
  Some(T),
}

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}

fn main() {
  let some_number = Some(5);
  let some_char = Some('E');

  // let absent_number: Option<i32> = None;

  // let x: i8 = 5;
  // let y: Option<i8> = Some(5);

  let penny = Coin::Penny;
  let quarter = Coin::Quarter(UsState::Alabama);

  value_in_cents(quarter);

  let five = Some(5);
  // let six = plus_one(five);
  // let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1)
  }
}