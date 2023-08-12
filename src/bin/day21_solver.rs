use advent_of_code_2022::day21::day21_model::{MathMonkeys, Monkey};
use advent_of_code_2022::input::parse_input_file;

fn main() ->Result<(),Box<dyn std::error::Error>> {
    let mut my_monkeys =MathMonkeys::new();
    let input = parse_input_file("src/day21/input.txt", &mut my_monkeys);
    println!("Day 21 part 1 ");
    let root = "root".to_string();
    println!("root says {}", my_monkeys.get_value_from_monkey(&root));

    println!("Day 21 part 2 ");
    my_monkeys.is_part2 = true;
    let humn_start = match my_monkeys.monkeys.get("humn").unwrap() {
        Monkey::Constant { value, .. } => {*value}
        _ => {panic!("Where am I??")}
    };
    my_monkeys.humn = humn_start;

    println!("root says {}",my_monkeys.get_value_from_monkey(&root));

    let (root_l,root_r) = match my_monkeys.monkeys.get("root").unwrap() {
        Monkey::Constant { .. } => {panic!("root can't be constant! ")}
        Monkey::Add { l_op,r_op, .. } => {(l_op,r_op)}
        Monkey::Sub { l_op,r_op, .. } => {(l_op,r_op)}
        Monkey::Mul { l_op,r_op, .. } => {(l_op,r_op)}
        Monkey::Div { l_op,r_op, .. } => {(l_op,r_op)}
    };
    let rtree = my_monkeys.get_value_from_monkey(root_r); // Will not change
    println!("rtree {}",rtree);

    let root_l = root_l.clone();
    let root_r= root_r.clone();

    let (v,s) = my_monkeys.print_expr(&root_r);
    match v {
        None => {println!("Expr: {}",s);}
        Some(v) => {println!("Expr {}",v);}
    }


    // my_monkeys.simplify(&root_l);
    // let mut count = 0;
    //
    // let (_,s) = my_monkeys.print_expr(&root_l);
    // println!("{}",s);

    let mut ltree = my_monkeys.get_value_from_monkey(&root_l);
    println!("ltree {}",ltree);
    println!("rtree-ltree {}",rtree-ltree);

    // Find
    // let mut last = 0;
    // for i in (-500000..500000).step_by(50000){
    //     my_monkeys.humn = i;
    //     let v = my_monkeys.get_value_from_monkey(&root_l);
    //     println!("{}, {} delta {}",i,v, v-last);
    //     last = v;
    // }


    while ltree > rtree {
        my_monkeys.humn = my_monkeys.humn * 2;
        ltree= my_monkeys.get_value_from_monkey(&root_l);
        println!("{}, {} delta {}",my_monkeys.humn,ltree,rtree-ltree);
    }
    println!("humn = {} gives left {} < {} right delta {}", my_monkeys.humn,ltree,rtree,rtree-ltree );
    let mut low = humn_start;
    let mut high = my_monkeys.humn;
    let mut count = 0;
    while ltree != rtree {
         my_monkeys.humn = low +  (high-low)/2;
        ltree= my_monkeys.get_value_from_monkey(&root_l);
        if ltree > rtree {
            low = my_monkeys.humn;
        }else{
            high = my_monkeys.humn;
        }

        count += 1;
        println!("{} ,humn {}, low {} high {}",(ltree-rtree).abs(),my_monkeys.humn,low,high);
        if count > 1000000 {
            panic!("to many iterations!");
        }
    }

    println!("I shout {} (after {} iterations)", my_monkeys.humn, count);
    my_monkeys.humn = my_monkeys.humn + 2;
    while ltree-rtree == 0 {
        my_monkeys.humn = my_monkeys.humn - 1;
        ltree = my_monkeys.get_value_from_monkey(&root_l);
        println!("humn {}  -> (ltree-rtree) {} ", my_monkeys.humn, (ltree - rtree).abs());
    }
    Ok(())
}


