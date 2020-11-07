#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
    input!(input: [(usize, usize); 100]);

    let re_input = rearrange(input);

    let mut x = 10;
    let mut y = 0;
    let mut command = "".to_string();

    for v in re_input {
        let s = get_move_command(x, y, v.0, v.1);
        command.push_str(&s);
        command.push_str("I");
        x = v.0;
        y = v.1;
    }

    print!("{}", command);
}

fn get_move_command(x: usize, y: usize, target_x: usize, target_y: usize) -> String {
    let x_command = if x < target_x { "D" } else { "U" };
    let y_command = if y < target_y { "R" } else { "L" };
    let mut command = "".to_string();
    for _i in 0..(x as isize - target_x as isize).abs() {
        command += x_command;
    }
    for _i in 0..(y as isize - target_y as isize).abs() {
        command += y_command;
    }

    command
}

fn rearrange(input: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut yamafuda = vec![];
    let inn = get_array(&input);

    let mut x = 0;
    let mut y = 0;
    let mut command = "".to_string();

    for (i, v) in inn {
        let s = get_move_command(x, y, v.0, v.1);
        command.push_str(&s);
        command.push_str("I");
        yamafuda.push(i);
        x = v.0;
        y = v.1;
    }
    let s = get_move_command(x, y, 19, 0);
    command.push_str(&s);

    let mut output = vec![];
    for i in 0..10 {
        let lr = i % 2 == 0;
        let move_side_command = if lr { "R" } else { "L" };

        for j in 0..10 {
            command += "O";

            output.push((yamafuda.pop(), (19 - i, if lr { j } else { 9 - j })));
            if j != 9 {
                command += move_side_command;
            }
        }
        if i != 9 {
            command += "U";
        }
    }
    print!("{}", command);
    output.sort_by_key(|x| x.0);

    output.iter().map(|x| x.1).collect()
}

fn get_array(input: &[(usize, usize)]) -> Vec<(usize, &(usize, usize))> {
    let mut asdf = vec![];
    for i in 0..20 {
        let lr = i % 2 == 0;
        let mut aaa: Vec<_> = input.iter().enumerate().filter(|x| (x.1).0 == i).collect();
        if lr {
            aaa.sort_by_key(|x| (x.1).1);
        } else {
            aaa.sort_by_key(|x| 19 - (x.1).1);
        }

        asdf.append(&mut aaa);
    }
    asdf
}
