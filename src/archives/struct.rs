struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

struct LiteralUser {
  active: bool,
  // username: &str,
  // email: &str,
  sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;


#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size
    }
  }
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  // let mut user = User {
  //   active: true,
  //   username: String::from("fmgono"),
  //   email: String::from("fmgonoo@gmail.com"),
  //   sign_in_count: 1
  // };

  // user.email = String::from("anotheremail@gmail.com");

  // let user1 = build_user(String::from("revmaghfira@gmail.com"), String::from("username"));
  // let user2 = build_user(user1.email, user.username);

  // let black = Color(0,0,0);
  // let origin = Point(0,0,0);

  // let subject = AlwaysEqual;

  // let literal_user = LiteralUser {
  //   active: true,
  //   email: "fmgonoo@gmail.com",
  //   username: "fmgono",
  //   sign_in_count: 1,
  // };
  let scale = 2;
  let rect1 = Rectangle {
    width: 30,
    height: 50
  };
  let rect2 = Rectangle {
    width: 10,
    height: 40
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45
  };

  let sq = Rectangle::square(4);

  // println!("The area of the rectangle is {} square pixels.", rect1.area());
  println!("The square: {:?}", sq);
  // dbg!(&rect1);

  println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
  

}

fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    email,
    username,
    sign_in_count: 1
  }
}