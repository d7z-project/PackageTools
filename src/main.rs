use boa_engine::Context;

fn main() {
    let mut context = Context::default();
    println!("{:?}", context.eval(r#"
    function hello(){
    console.log("hello world");
    return 1;
    }
    "#).unwrap());
    println!("{:?}", context.eval("hello()").unwrap());
}
