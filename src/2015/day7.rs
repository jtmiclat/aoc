use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Wire<'a> {
    source: Vec<&'a str>,
}
impl Wire<'_> {
    pub fn parse_and_calculate(
        wire_name: &str,
        graph: &HashMap<&str, Wire>,
        cache: &mut HashMap<String, u16>,
    ) -> u16 {
        if cache.get(&wire_name.to_owned()).is_some() {
            return cache[&wire_name.to_owned()];
        }
        let result = match graph.get(wire_name) {
            Some(wire) => wire.calculate(graph, cache),
            None => wire_name.to_string().parse::<u16>().expect("what"),
        };
        cache.insert(wire_name.to_owned(), result);
        result
    }
    pub fn calculate(&self, graph: &HashMap<&str, Wire>, cache: &mut HashMap<String, u16>) -> u16 {
        if self.source.len() == 3 {
            return Wire::parse_and_calculate(self.source[0].clone(), graph, cache);
        }
        if self.source[0] == "NOT" {
            return !Wire::parse_and_calculate(self.source[1].clone(), graph, cache);
        }
        let arg_1: u16 = Wire::parse_and_calculate(self.source[0].clone(), graph, cache);
        let arg_3: u16 = Wire::parse_and_calculate(self.source[2].clone(), graph, cache);
        let x = match self.source[1] {
            "LSHIFT" => arg_1 << arg_3,
            "RSHIFT" => arg_1 >> arg_3,
            "AND" => arg_1 & arg_3,
            "OR" => arg_1 | arg_3,
            _ => panic!("Error"),
        };
        return x;
    }
}
pub fn part1(file: String) {
    let value: String = fs::read_to_string(file).expect("Error opening file");
    let commands: Vec<&str> = value.split("\n").map(|x| x.clone()).collect();
    let mut graph: HashMap<&str, Wire> = HashMap::new();
    let mut cache: HashMap<String, u16> = HashMap::new();
    for command in commands {
        let args: Vec<&str> = command.split_whitespace().map(|x| x.clone()).collect();
        let wire_name = args[args.len() - 1];
        let wire = Wire { source: args };
        graph.insert(wire_name, wire);
    }
    let wire_a = &graph["a"];
    println!("Value of a {:?}", wire_a.calculate(&graph, &mut cache));
}
#[allow(unused_variables)]
pub fn part2(file: String) {
    println!("Same solution as part 1 using day 7b.txt")
    // Real solution would be run part 1.
    // Change line with -> b
    // Rerun solution again
}
