use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

const MIN_RATING: usize = 1;
const MAX_RATING: usize = 4000;

#[derive(Debug, PartialEq)]
enum Target {
    Reject,
    Accept,
    Workflow(String),
}

impl FromStr for Target {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Target::Reject),
            "A" => Ok(Target::Accept),
            name => Ok(Target::Workflow(name.to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => unimplemented!(),
        }
    }
}

impl Category {
    fn as_num(&self) -> usize {
        match self {
            Category::X => 0,
            Category::M => 1,
            Category::A => 2,
            Category::S => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Compare {
    Lower,
    Greater,
}

impl Compare {
    fn check(&self, value: usize, other: usize) -> bool {
        match self {
            Compare::Lower => value < other,
            Compare::Greater => value > other,
        }
    }
}

impl From<char> for Compare {
    fn from(value: char) -> Self {
        match value {
            '>' => Compare::Greater,
            '<' => Compare::Lower,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Workflow {
    rules: Vec<Rule>,
    otherwise: Target,
}

#[derive(Debug, PartialEq)]
struct Rule {
    compare: Compare,
    category: Category,
    other: usize,
    if_true: Target,
}

impl Rule {
    fn check(&self, other: &Part) -> bool {
        self.compare
            .check(other.get_cat(&self.category), self.other)
    }
}

#[derive(Debug, PartialEq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn get_cat(&self, cat: &Category) -> usize {
        match cat {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn from_categories(cats: Vec<(&str, usize)>) -> Self {
        let mut x = 0;
        let mut m = 0;
        let mut a = 0;
        let mut s = 0;

        for (name, value) in cats {
            match name {
                "x" => {
                    x = value;
                }
                "m" => {
                    m = value;
                }
                "a" => {
                    a = value;
                }
                "s" => {
                    s = value;
                }
                _ => unimplemented!(),
            }
        }

        Self { x, m, a, s }
    }
}

fn main() {
    let input = Path::new("2023/day19/src/input.txt");
    let contents = fs::read_to_string(input).unwrap();
    let (workflows, parts) = parse_input(&contents);

    let part1 = std::time::Instant::now();
    println!("Part 1");
    println!(
        "Sum of ratings of accepted parts: {:?}",
        sort_parts(parts, &workflows)
    );
    println!("In {:?}", part1.elapsed());

    let part2 = std::time::Instant::now();
    println!("Part 2");
    println!(
        "Unique accepted combinations of ratings: {:?}",
        unique_rating_combinations(&workflows)
    );
    println!("In {:?}", part2.elapsed());
}

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let mut lines = input.lines();
    let mut workflows = HashMap::new();

    let mut parts = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut workflow_rules = Vec::new();

        let (name, rest) = line.split_once("{").unwrap();
        let rules: Vec<_> = rest[..rest.len() - 1].split(",").collect();

        for rule in &rules[0..(rules.len() - 1)] {
            let (rule_rest, target) = rule.split_once(":").unwrap();
            let split: Vec<_> = rule_rest.split_inclusive(['>', '<']).collect();

            let cat_with_comp: Vec<_> = split[0].chars().collect();
            let category = Category::from(cat_with_comp[0]);
            let compare = Compare::from(cat_with_comp[1]);

            workflow_rules.push(Rule {
                category,
                compare,
                other: split.last().unwrap().parse().unwrap(),
                if_true: target.parse().unwrap(),
            });
        }

        workflows.insert(
            name,
            Workflow {
                rules: workflow_rules,
                otherwise: Target::from_str(rules.last().unwrap()).unwrap(),
            },
        );
    }

    for line in lines {
        let categories: Vec<(&str, usize)> = line[1..(line.len() - 1)]
            .split(',')
            .map(|c| {
                let (name, value) = c.split_once("=").unwrap();
                (name, value.parse().unwrap())
            })
            .collect();
        parts.push(Part::from_categories(categories))
    }
    (workflows, parts)
}

fn sort_parts(parts: Vec<Part>, workflows: &HashMap<&str, Workflow>) -> usize {
    let mut accepted: Vec<Part> = Vec::new();

    for part in parts {
        let mut workflow_name = "in";

        'outer: loop {
            let workflow = workflows.get(&workflow_name).unwrap();
            for rule in &workflow.rules {
                if rule.check(&part) {
                    match &rule.if_true {
                        Target::Accept => {
                            accepted.push(part);
                            break 'outer;
                        }
                        Target::Workflow(name) => {
                            workflow_name = name.as_str();
                            continue 'outer;
                        }
                        Target::Reject => {
                            break 'outer;
                        }
                    }
                }
            }
            match &workflow.otherwise {
                Target::Accept => {
                    accepted.push(part);
                    break;
                }
                Target::Workflow(name) => {
                    workflow_name = name.as_str();
                }
                Target::Reject => {
                    break;
                }
            }
        }
    }

    accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum()
}

fn unique_rating_combinations(workflows: &HashMap<&str, Workflow>) -> usize {
    let mut stack = Vec::new();
    stack.push(("in", [(MIN_RATING, MAX_RATING); 4]));
    let mut unique_accepted = 0;

    while let Some((workflow_name, mut ratings)) = stack.pop() {
        let workflow = workflows.get(workflow_name).unwrap();

        for rule in &workflow.rules {
            let c_num = rule.category.as_num();
            let mut next_ratings = ratings;
            match rule.compare {
                Compare::Lower => {
                    next_ratings[c_num].1 = next_ratings[c_num].1.min(rule.other - 1);
                    ratings[c_num].0 = ratings[c_num].0.max(rule.other);
                }
                Compare::Greater => {
                    next_ratings[c_num].0 = next_ratings[c_num].0.max(rule.other + 1);
                    ratings[c_num].1 = ratings[c_num].1.min(rule.other);
                }
            }
            match &rule.if_true {
                Target::Accept => {
                    unique_accepted += count_combinations_from_ranges(&next_ratings);
                }
                Target::Workflow(name) => {
                    stack.push((name, next_ratings));
                }
                Target::Reject => {}
            }
        }

        match &workflow.otherwise {
            Target::Accept => {
                unique_accepted += count_combinations_from_ranges(&ratings);
            }
            Target::Workflow(name) => {
                stack.push((name, ratings));
            }
            Target::Reject => {}
        }
    }
    unique_accepted
}

fn count_combinations_from_ranges(ranges: &[(usize, usize); 4]) -> usize {
    ranges.iter().map(|(low, high)| *high - *low + 1).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parse_input() {
        let (workflows, parts) = parse_input(SAMPLE);

        assert_eq!(
            parts,
            [
                Part {
                    x: 787,
                    m: 2655,
                    a: 1222,
                    s: 2876
                },
                Part {
                    x: 1679,
                    m: 44,
                    a: 2067,
                    s: 496
                },
                Part {
                    x: 2036,
                    m: 264,
                    a: 79,
                    s: 2244
                },
                Part {
                    x: 2461,
                    m: 1339,
                    a: 466,
                    s: 291
                },
                Part {
                    x: 2127,
                    m: 1623,
                    a: 2188,
                    s: 1013
                },
            ]
        );

        assert_eq!(
            *workflows.get("px").unwrap(),
            Workflow {
                rules: Vec::from([
                    Rule {
                        category: Category::A,
                        compare: Compare::Lower,
                        other: 2006,
                        if_true: Target::Workflow("qkq".into())
                    },
                    Rule {
                        category: Category::M,
                        compare: Compare::Greater,
                        other: 2090,
                        if_true: Target::Accept
                    },
                ]),
                otherwise: Target::Workflow("rfg".into())
            }
        );
    }

    #[test]
    fn test_part_1_sample() {
        let (workflows, parts) = parse_input(SAMPLE);

        assert_eq!(sort_parts(parts, &workflows), 19114);
    }

    #[test]
    fn test_part_2_sample() {
        let (workflows, _) = parse_input(SAMPLE);

        assert_eq!(unique_rating_combinations(&workflows), 167409079868000);
    }
}
