use solution::Solution;


mod solution;
mod gui;
mod diff;

fn main() {
    let values = get_data();
    let ranks = Solution::matrix_rank_transform(values.clone());
    let m = values.len();
    let n = values[0].len();

    for i in 0 .. m {
        print!("|");
        for j in 0 .. n {
            print!("{: >2}|", ranks[i][j]);
        }
        println!();
    }

    println!("diff {}", diff::result(&ranks));

    let uf = Solution::union_find(&values);

    gui::run(ranks, values, diff::data(), uf);

}

fn from_array<const M: usize, const N: usize>(arr: [[i32; N]; M]) -> Vec<Vec<i32>> {
    <[_; M]>::into_iter(arr).map(Vec::from).collect()
}

fn get_data() -> Vec<Vec<i32>> {
    // from_array([[1,2],[3,4]])
    // from_array([[7,7],[7,7]])
    // from_array([[20,-21,14],[-19,4,19],[22,-47,24],[-19,4,19]])
    // from_array([[-32,15,38,17,-44,43,42,-47,-44,-41],[34,-43,-24,7,-10,-43,36,-5,-22,37],[4,-13,-38,49,16,-21,30,13,-20,47],[2,-35,32,11,26,-31,40,31,-46,-7],[4,19,18,-27,16,43,-10,-11,44,39],[18,9,48,-29,30,5,8,-13,-42,-43],[48,47,30,29,24,-29,22,-31,12,-37],[38,-23,44,-13,-46,37,-12,31,14,-31],[-28,23,-50,-23,12,23,18,-11,-44,31],[-10,37,16,11,-18,17,40,-41,26,-31]])
    // from_array([[25,8,31,42,-39,8,31,-10,33,-44,7,-30,9,44,15,26],[-3,-48,-17,-18,9,-12,-21,10,1,44,-17,14,-27,48,-21,-6],[49,28,27,-18,-31,4,-13,34,49,48,47,-18,33,40,15,38],[5,-28,-49,-38,1,32,-25,-50,29,-32,35,-46,-43,48,-49,-6],[-27,-24,23,-14,-47,-12,7,6,25,-16,47,-26,13,-12,-33,-18],[45,-48,3,-26,-23,-36,-17,38,17,12,15,46,37,40,47,26],[-19,-24,-21,-2,-7,-48,47,30,5,-8,23,-46,21,-32,-33,-26],[-27,32,27,-26,21,-32,-49,-10,5,20,-29,46,-43,-44,39,22],[-43,48,27,26,-27,12,-1,-10,-27,12,-29,-34,41,-28,-25,-30],[25,-36,35,-26,37,-20,31,14,-19,-40,-29,-2,-39,-28,11,46],[49,-32,-29,-6,-47,32,-17,-18,-23,24,23,22,-47,-44,27,14],[37,-44,-33,-18,-47,24,-17,-46,-43,-32,15,-46,-27,-8,-25,46],[41,-40,31,-30,13,-24,-29,22,-15,-16,47,2,-39,4,-25,-42],[-3,12,7,14,-7,8,-37,-34,-7,-12,39,-38,1,44,27,-34],[-47,4,7,-2,-43,-32,27,2,-43,-8,-33,14,49,-48,-5,30],[-15,8,-33,-26,-23,-32,-25,22,13,-20,-9,26,29,4,-1,2]])
    from_array(include!("../data.json"))
}
