mod rules;
use rules::Rules;
mod read_file;
use read_file::read_file;

const FILE: bool = false;

fn main() {
    let rules: Rules = Rules {
        insertion: 1.,
        deletion: 1.,
        substitution: 1.,
        transposition: 1.,
    };

    let mut query: String = String::new();
    let mut target: String= String::new();

    if FILE {
        (query, target) = read_file("input.txt");
    }

}
