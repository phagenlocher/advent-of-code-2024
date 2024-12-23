use itertools::Itertools;
use petgraph::prelude::UnGraphMap;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::{DefaultHasher, Hash, Hasher},
};

advent_of_code::solution!(23);

struct AllPairs<I: Iterator> {
    elements: VecDeque<I::Item>,
    curr: Option<I::Item>,
    i: usize,
}

impl<I: Iterator> AllPairs<I> {
    fn all_pairs(iter: I) -> AllPairs<I> {
        AllPairs {
            elements: iter.collect(),
            curr: None,
            i: 0,
        }
    }
}

impl<I: Iterator> Iterator for AllPairs<I>
where
    I::Item: Copy,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        if self.elements.len() <= self.i {
            self.i = 0;
            self.curr = None;
        }

        if self.curr.is_none() {
            self.curr = self.elements.pop_front();
        }

        if self.elements.is_empty() {
            return None;
        }

        match self.curr {
            None => None,
            Some(a) => {
                let b = self.elements[self.i];
                self.i += 1;
                Some((a, b))
            }
        }
    }
}

fn parse_input(input: &str) -> UnGraphMap<&str, ()> {
    let edges = input.split("\n").filter_map(|line| {
        if let [from, to] = line.split("-").collect::<Vec<_>>()[..] {
            Some((from, to))
        } else {
            None
        }
    });
    UnGraphMap::from_edges(edges)
}

fn sorted_three_tuple<T: Ord>(a: T, b: T, c: T) -> (T, T, T) {
    if a >= b && a >= c {
        if b >= c {
            (a, b, c)
        } else {
            (a, c, b)
        }
    } else if b >= a && b >= c {
        if a >= c {
            (b, a, c)
        } else {
            (b, c, a)
        }
    } else {
        if a >= b {
            (c, a, b)
        } else {
            (c, b, a)
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let g = parse_input(input);
    let mut counted = HashSet::new();
    for node in g.nodes() {
        if node.starts_with("t") {
            let nbrs = g.neighbors(node);
            for (nbr1, nbr2) in AllPairs::all_pairs(nbrs) {
                if g.neighbors(nbr1).collect::<HashSet<_>>().contains(nbr2) {
                    counted.insert(sorted_three_tuple(node, nbr1, nbr2));
                }
            }
        }
    }
    Some(counted.len())
}

#[derive(Debug, Clone)]
struct Cluster<N: Eq + Hash> {
    elems: HashSet<N>,
}

impl<N: Eq + Ord + Hash + Clone> Cluster<N> {
    fn insert(&mut self, node: N) {
        self.elems.insert(node);
    }

    fn subclusters(&self) -> HashSet<Cluster<N>> {
        (2..=self.elems.len())
            .map(|n| self.subclusters_of_size(n))
            .flatten()
            .collect()
    }

    fn subclusters_of_size(&self, count: usize) -> HashSet<Cluster<N>> {
        self.elems
            .clone()
            .into_iter()
            .combinations(count)
            .map(|combs| combs.into_iter().collect::<Cluster<_>>())
            .collect::<HashSet<Cluster<_>>>()
    }
}

impl<N: Eq + Hash> FromIterator<N> for Cluster<N> {
    fn from_iter<T: IntoIterator<Item = N>>(iter: T) -> Self {
        let elems = iter.into_iter().collect::<HashSet<_>>();
        Cluster { elems }
    }
}

impl<N: Eq + Ord + Hash> Hash for Cluster<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut elems = self.elems.iter().collect::<Vec<_>>();
        elems.sort();
        for elem in elems {
            elem.hash(state);
        }
    }
}

impl<N: Eq + Ord + Hash> PartialEq for Cluster<N> {
    fn eq(&self, other: &Self) -> bool {
        let mut hasher1 = DefaultHasher::new();
        self.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        other.hash(&mut hasher2);
        hasher1.finish() == hasher2.finish()
    }
}

impl<N: Eq + Ord + Hash> Eq for Cluster<N> {
    fn assert_receiver_is_total_eq(&self) {}
}

fn get_clusters<N: PartialEq + Ord + Copy + Hash>(
    graph: &UnGraphMap<N, ()>,
) -> impl Iterator<Item = Cluster<N>> + '_ {
    graph.nodes().map(|node| {
        let mut cluster = graph.neighbors(node).collect::<Cluster<_>>();
        cluster.insert(node);
        cluster
    })
}

fn fully_connected_components<N>(g: &UnGraphMap<N, ()>) -> impl Iterator<Item = Cluster<N>> + '_
where
    N: Debug + PartialEq + Eq + Ord + Hash + Clone + Copy,
{
    fn count_elem<T: PartialEq>(vec: &Vec<T>, elem: &T) -> usize {
        vec.iter().filter(|x| *x == elem).count()
    }

    let mut cluster_count_map: HashMap<Cluster<_>, usize> = HashMap::new();
    for cluster in get_clusters(g)
        .collect::<Vec<_>>()
        .iter()
        .map(|c| c.subclusters())
        .flatten()
    {
        cluster_count_map
            .entry(cluster)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    cluster_count_map
        .into_iter()
        .filter_map(|(cluster, count)| {
            if cluster.elems.len() == count {
                Some(cluster)
            } else {
                None
            }
        })
}

pub fn part_two(input: &str) -> Option<String> {
    let g = parse_input(input);

    let mut components = fully_connected_components(&g).collect::<Vec<_>>();
    components.sort_by(|a, b| a.elems.len().cmp(&b.elems.len()));

    if let Some(cluster) = components.last() {
        let mut nodes = cluster.clone().elems.into_iter().collect::<Vec<_>>();
        nodes.sort();
        let result = nodes
            .into_iter()
            .intersperse(",")
            .collect::<Vec<_>>()
            .concat();
        return Some(result);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_pairs() {
        let input = vec![1, 2, 3];
        let result = AllPairs::all_pairs(input.iter()).collect::<Vec<_>>();
        assert_eq!(result, vec![(&1, &2), (&1, &3), (&2, &3)]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_cluster() {
        let cluster0 = [1, 3, 4].into_iter().collect::<Cluster<_>>();
        let cluster1 = [1, 2, 3].into_iter().collect::<Cluster<_>>();
        let mut cluster2 = [3, 1].into_iter().collect::<Cluster<_>>();
        cluster2.insert(2);
        assert_eq!(cluster1, cluster2);
        assert_ne!(cluster0, cluster1);
        assert_ne!(cluster0, cluster2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
