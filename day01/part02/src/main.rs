use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Tide {
    Increased,
    Decreased
}

fn find_pair_increased(s: Vec<i64>) -> Vec<Tide> {
    dbg!(&s[..5]);
    let mut depth: Vec<Tide> = Vec::new();
    for i in 1..s.len() {
            if s[i] - s[i-1] > 0 {
                depth.push(Tide::Increased);
            }
            else {
                depth.push(Tide::Decreased);
            }
    }
    depth
}

fn find_triple_increased(s: Vec<i64>) -> Vec<Tide> {
    let mut depth: Vec<Tide> = Vec::new();
    let triplet = s.iter().tuple_windows::<(_, _, _)>()
                                                            .map(|(a, b, c)| a+b+c)
                                                            .collect::<Vec<i64>>();

    for i in 1..triplet.len() {
        if triplet[i] - triplet[i-1] > 0 {
            depth.push(Tide::Increased);
        }
        else {
            depth.push(Tide::Decreased);
        }
    }

    depth
}

fn main() -> anyhow::Result<()> {
    // let s = std::fs::read_to_string("input.txt")?;
    let s = include_str!("input.txt").trim_end();
    let s = s.split('\n').map(str::parse::<i64>)
                                                    // .map(Result::unwrap)
                                                    .collect::<Result<Vec<_>, _>>()?;
    // dbg!(&s[..5]);
    
    // next line refers to part 01
    // let depth = find_pair_increased(s);

    // next line refers to part 02
    let depth = find_triple_increased(s);


    let larger_than_previous = depth.into_iter().filter(|x| *x == Tide::Increased).count();
    // dbg!(&depth[..5]);
    // dbg!(s.next().unwrap());
    // dbg!(s.next().unwrap());

    dbg!(larger_than_previous);
    Ok(())
}
