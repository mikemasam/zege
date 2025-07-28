fn main(){
    let user = User { name: "hi".to_owned() };
    app.run_before_start(|| print_user(user));
    app.run_after_start(|| print_user(user));
}
#[derive(Debug)]
struct User {
    name: String
}
fn print_user(User user){
    println!("Hi {}", user.name);
}
