#[derive(Copy, Clone)]
enum Primitive {
    Add,
    Sub,
    Multiply,
    Number(i32),
}

fn evaluate(primitives: Vec<Primitive>) -> i32 {
    let firstElement = &primitives[0];
    let mut iter = primitives.iter();
    iter.next();
    match firstElement {
        Primitive::Add => iter.fold(0, |t, n| t + evaluate(vec![*n])),
        Primitive::Sub => {
            let mut diff = evaluate(vec![*iter.next().unwrap()]);
            iter.fold(diff, |t, n| t - evaluate(vec![*n]))
        }
        Primitive::Multiply => iter.fold(1, |p, n| p * evaluate(vec![*n])),
        Primitive::Number(val) => *val,
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Sub);
    primitives.push(Primitive::Number(20));
    primitives.push(Primitive::Number(4));
    primitives.push(Primitive::Number(5));
    let result = evaluate(primitives);
    println!("The result is {result}");

    let mut pprimitives = Vec::<Primitive>::new();
    pprimitives.push(Primitive::Add);
    pprimitives.push(Primitive::Number(20));
    pprimitives.push(Primitive::Number(5));
    pprimitives.push(Primitive::Number(4));
    let product = evaluate(pprimitives);
    println!("The result is {product}");
}
