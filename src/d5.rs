use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let mut file = File::open("./test_files/d5_input1.txt").expect("Failed to open file");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("Failed to read file");

    let mut buffer = buffer.lines();

   let order_rules: HashMap<u8, HashSet<u8>> = 
        buffer.by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut ordering = 
                    line.split("|")
                        .map(|page| page.parse::<u8>().unwrap());
                (ordering.next().unwrap(), ordering.next().unwrap())     
            })
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.entry(key)
                    .or_insert_with(HashSet::new)
                    .insert(value);
                acc
            });


    let updates: Vec<Vec<u8>> =
        buffer.filter(|line| !line.is_empty())
            .map(|line| {
                line.split(",")
                    .map(|page| page.parse::<u8>().unwrap())
                    .collect()
            })
        .collect();

    let mut page_sum = 0;
    for mut update in updates {
        if !is_valid_update(&update, &order_rules) {
            fix_update(&mut update, &order_rules);
            page_sum += update[update.len() / 2] as u32;
        }
    }

    println!("{}", page_sum);
}

fn fix_update(update: &mut Vec<u8>, order_rules: &HashMap<u8, HashSet<u8>>) {
    let mut i = 0;
    'outer: while i < update.len() {
        let page = update[i];
        let children = match order_rules.get(&page) {
            Some(children) => children,
            None => continue
        };
        
        for child in children {
            if let Some(position) = update[0..i].iter().position(|p| p == child) {
                update.remove(i);
                update.insert(position, page);
                i = 0;
                continue 'outer;
            }
        }
        i += 1;
    }
}

fn is_valid_update(update: &Vec<u8>, order_rules: &HashMap<u8, HashSet<u8>>) -> bool {
    for (i, page) in update.iter().enumerate() {
        let prev_pages: HashSet<u8> = HashSet::from_iter(update[0..i].iter().cloned());
        let empty_set = HashSet::new();
        let children = order_rules.get(page).unwrap_or_else(|| &empty_set);
        if !prev_pages.intersection(children).collect::<Vec<_>>().is_empty() {
            return false
        }
    }
    true
}

