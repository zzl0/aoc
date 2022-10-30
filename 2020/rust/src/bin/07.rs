use itertools::Itertools;
use multimap::MultiMap;
use std::fmt;

/// (adjective, color), i.e. ("dark", "orange")
type BagSpec<'a> = (&'a str, &'a str);

/// K can contain V.0 of V.1
type Rules<'a> = MultiMap<BagSpec<'a>, (usize, BagSpec<'a>)>;

struct FormattedRules<'a>(Rules<'a>);

impl fmt::Display for FormattedRules<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, vv) in &self.0 {
            write!(f, "{} {} bags can contain ", k.0, k.1)?;
            if vv.is_empty() {
                write!(f, "no other bags")?;
            } else {
                for (i, v) in vv.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(
                        f,
                        "{} {} {} {}",
                        v.0,
                        v.1 .0,
                        v.1 .1,
                        if v.0 == 1 { "bag" } else { "bags" }
                    )?;
                }
            }
            writeln!(f, ".")?;
        }
        Ok(())
    }
}

fn parse_rules(input: &str) -> Rules<'_> {
    let mut rules = Default::default();

    peg::parser! {
        pub(crate) grammar parser() for str {
            pub(crate) rule root(r: &mut Rules<'input>)
                = (line(r) "." whitespace()*)* ![_]

            rule line(r: &mut Rules<'input>)
                = spec:bag_spec() " contain " rules:rules() {
                    if let Some(rules) = rules {
                        for rule in rules {
                            r.insert(spec, rule)
                        }
                    }
                }

            rule bag_spec() -> BagSpec<'input>
                = adjective:name() " " color:name() " bag" "s"? { (adjective, color) }

            rule rules() -> Option<Vec<(usize, BagSpec<'input>)>>
                = rules:rule1()+ { Some(rules) }
                / "no other bags" { None }

            rule rule1() -> (usize, BagSpec<'input>)
                = r:rule0() ", "? { r }

            rule rule0() -> (usize, BagSpec<'input>)
                = quantity:number() " " spec:bag_spec() { (quantity, spec) }

            rule number() -> usize
                = e:$(['0'..='9']+) { e.parse().unwrap() }

            rule name() -> &'input str
                = $((!whitespace()[_])*)

            rule whitespace()
                = [' ' | '\t' | '\r' | '\n' ]
        }
    }

    parser::root(input, &mut rules).unwrap();
    rules
}

#[allow(dead_code)]
fn subgraph_contains(graph: &Rules<'_>, root: &(&str, &str), needle: &(&str, &str)) -> bool {
    if let Some(neighbors) = graph.get_vec(root) {
        for (_, neighbor) in neighbors {
            if neighbor == needle || subgraph_contains(graph, neighbor, needle) {
                return true;
            }
        }
    }
    false
}

fn reverse_graph<'a>(graph: &Rules<'a>) -> Rules<'a> {
    let mut reverse: Rules = Default::default();
    for (&node, neighbors) in graph.iter_all() {
        for &(quantity, neighbor) in neighbors {
            reverse.insert(neighbor, (quantity, node));
        }
    }
    reverse
}

#[allow(dead_code)]
fn walk_subgraph<'a>(graph: &Rules<'a>, root: &(&str, &str)) -> Vec<(&'a str, &'a str)> {
    let mut res: Vec<_> = Default::default();
    if let Some(neighbors) = graph.get_vec(root) {
        for &(_quantity, neighbor) in neighbors {
            res.push(neighbor);
            res.extend(walk_subgraph(graph, &neighbor));
        }
    }
    res
}

#[allow(dead_code)]
fn walk_subgraph1<'a>(graph: &Rules<'a>, root: &(&str, &str), res: &mut Vec<(&'a str, &'a str)>) {
    if let Some(neighbors) = graph.get_vec(root) {
        for &(_quantity, neighbor) in neighbors {
            res.push(neighbor);
            walk_subgraph1(graph, &neighbor, res);
        }
    }
}

// http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch19-02-advanced-lifetimes.html
// 'elems is at least as long as 'iter
fn walk_subgraph2<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = (&'elems str, &'elems str)> + 'iter> {
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .flat_map(move |&(_, neighbor)| {
                std::iter::once(neighbor).chain(walk_subgraph2(graph, &neighbor))
            }),
    )
}

fn part1(rules: &Rules<'_>) {
    let needle = &("shiny", "gold");
    // let colors_that_contain_shiny_gold: Vec<_> = rules
    //     .keys()
    //     .filter(|&k| k != needle)
    //     .filter(|&k| subgraph_contains(rules, k, needle))
    //     .collect();

    // let reverse_rules = reverse_graph(rules);
    // let colors_that_contain_shiny_gold = walk_subgraph(&reverse_rules, needle);

    let reverse_rules = reverse_graph(rules);
    let colors_that_contain_shiny_gold = walk_subgraph2(&reverse_rules, needle).unique().count();

    println!("part1: {:?}", colors_that_contain_shiny_gold);
}

#[allow(dead_code)]
fn walk_subgraph3<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = (usize, (&'elems str, &'elems str))> + 'iter> {
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .flat_map(move |&n| std::iter::once(n).chain(walk_subgraph3(graph, &n.1))),
    )
}

fn bag_quantities<'iter, 'elems: 'iter>(
    graph: &'iter Rules<'elems>,
    root: &(&'iter str, &'iter str),
) -> Box<dyn Iterator<Item = usize> + 'iter> {
    Box::new(
        graph
            .get_vec(root)
            .into_iter()
            .flatten()
            .flat_map(move |&(qt, n)| {
                std::iter::once(qt).chain(bag_quantities(graph, &n).map(move |x| x * qt))
            }),
    )
}

fn part2(rules: &Rules<'_>) {
    let root = &("shiny", "gold");
    let answer: usize = bag_quantities(rules, root).sum();
    println!("part2: {}", answer);
}

fn main() {
    let input = include_str!("../../../data/day07.txt");
    let rules = parse_rules(input);
    part1(&rules);
    part2(&rules);
}
