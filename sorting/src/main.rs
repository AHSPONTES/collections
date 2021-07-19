use std::vec::Vec;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DVD {
    title: String,
    year: u32,
}

impl DVD {
    pub fn new(title: String, year: u32) -> Self {
        DVD { title, year }
    }
}

fn main() {
    let mut movies = vec![
        DVD::new("Buckaroo Banzai Across the 8th Dimension".to_string(), 1984),
        DVD::new("Captain America".to_string(), 2011),
        DVD::new("Stargate".to_string(), 1994),
        DVD::new("When Harry Met Sally".to_string(), 1989),
        DVD::new("Kiss Kiss Bang Bang".to_string(), 2005),
        DVD::new("The Dark Knight".to_string(), 2008),
        DVD::new("Boys Night Out".to_string(), 1962),
        DVD::new("The Glass Bottom Boat".to_string(), 1966),
    ];

    movies.sort_by(|a, b| b.year.cmp(&a.year));

    while let Some(movie) = movies.pop() {
        println!("{:?}", movie);
    }

    /*  sorting vec stack
        let mut sort_stack = Vec::new();
        sort_stack.push("anteater");
        sort_stack.push("zebra");
        sort_stack.push("tapir");
        sort_stack.push("elephant");
        sort_stack.push("coati");
        sort_stack.push("leopard");
        sort_stack.sort();

        while let Some(element) = sort_stack.pop() {
            println!("{}", element);
        }
    */
}
