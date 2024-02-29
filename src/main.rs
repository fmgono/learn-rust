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
    width: dbg!(30 * scale),
    height: 30
  };

  // println!("The area of the rectangle is {} square pixels.", area(&rect1));
  println!("The rectangle: {:?}", rect1);
  dbg!(&rect1);

}

fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    email,
    username,
    sign_in_count: 1
  }
}

fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}