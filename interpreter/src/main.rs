enum Primitive {
    Sub,
    Add,
    Multiply,
    Number(i32),
}

fn eval_prim(primitive: &Primitive) -> i32 {
    match primitive {
        Primitive::Number(val) => *val,
        _ => 0,
    }
}

fn evaluate(primitives: Vec<Primitive>) -> i32 {
    match primitives[0] {
        Primitive::Add => eval_prim(&primitives[1]) + eval_prim(&primitives[2]),
        Primitive::Sub => eval_prim(&primitives[1]) - eval_prim(&primitives[2]),
        Primitive::Multiply => eval_prim(&primitives[1]) * eval_prim(&primitives[2]),
        Primitive::Number(_) => 0,
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Multiply);
    primitives.push(Primitive::Number(3));
    primitives.push(Primitive::Number(4));
    let result = evaluate(primitives);
    println!("The product is {result}");

    let mut sum_primitives = Vec::<Primitive>::new();
    sum_primitives.push(Primitive::Add);
    sum_primitives.push(Primitive::Number(3));
    sum_primitives.push(Primitive::Number(4));
    let sum = evaluate(sum_primitives);

    println!("the sum is {sum}");

    let mut difference_primitives = Vec::<Primitive>::new();
    difference_primitives.push(Primitive::Sub);
    difference_primitives.push(Primitive::Number(3));
    difference_primitives.push(Primitive::Number(4));
    let difference = evaluate(difference_primitives);
    println!("the difference is {difference}");
}
