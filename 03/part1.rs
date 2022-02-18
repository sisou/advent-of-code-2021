fn main() {
    let input = utils::load_input("./input.txt");

    let mut weights: [i32; 12] = Default::default();

    for line in input {
        let chars: Vec<char> = line.chars().collect();

        for i in 0..weights.len() {
            match chars[i] {
                '1' => weights[i] += 1,
                '0' => weights[i] -= 1,
                _ => panic!("Invalid character"),
            }
        }
    }

    /*
     * At first, I implemented the bits of gamma and epsilon as I knew how, with an array of chars,
     * joined to a string, then parsed as binary. The commented-out code below works and got me the
     * right answer.
     */

    // let mut gamma_chars = vec!['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];
    // let mut epsilon_chars = vec!['0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0', '0'];
    // for i in 0..weights.len() {
    //     if weights[i] > 0 {
    //         // Gamma rate is MOST COMMON bits
    //         gamma_chars[i] = '1';
    //     } else {
    //         // Epsilon rate is LEAST COMMON bits
    //         epsilon_chars[i] = '1';
    //     }
    // }

    // let gamma_binary: String = gamma_chars.into_iter().collect();
    // let gamma = i32::from_str_radix(&gamma_binary, 2).unwrap();

    // let epsilon_binary: String = epsilon_chars.into_iter().collect();
    // let epsilon = i32::from_str_radix(&epsilon_binary, 2).unwrap();

    /*
     * But then I thought, how would it be solved "cleanly", with bits and bitwise operators? The code
     * below the commented-out code is the result.
     */

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..weights.len() {
        let position = 0b100000000000 >> i;
        if weights[i] > 0 {
            gamma |= position;
        } else {
            epsilon |= position;
        }
    }

    println!("Gamma rate: {}", gamma);
    println!("Epsilon rate: {}", epsilon);
    println!("Power consumption: {}", gamma * epsilon);
}
