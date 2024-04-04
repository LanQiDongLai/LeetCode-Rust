use std::collections::BTreeSet;

struct Solution;
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut line_edges: Vec<Vec<i32>> = Vec::new();
        line_edges.resize(n as usize, Vec::new());
        for edge in edges{
            line_edges[edge[1] as usize].push(edge[0]);
        }
        let mut nodes_ancestors:Vec<Option<Vec<i32>>> = Vec::new();
        nodes_ancestors.resize(n as usize, None);
        for node in 0..n{
            Self::find_ancenstor(node, &line_edges, &mut nodes_ancestors);
        }
        nodes_ancestors.iter().map(|ancestors| ancestors.clone().unwrap()).collect()
    }
    fn find_ancenstor(node: i32, line_edges: &Vec<Vec<i32>>, nodes_ancestors: &mut Vec<Option<Vec<i32>>>){
        if line_edges.is_empty(){
            nodes_ancestors[node as usize] = Some(vec![]);
            return;
        }
        let mut ancestor_set = BTreeSet::new();
        for parent in &line_edges[node as usize]{
            if let Some(parent_ancestors) = &nodes_ancestors[*parent as usize]{
                for ancestor in parent_ancestors{
                    ancestor_set.insert(*ancestor);
                }
            }
            else{
                Self::find_ancenstor(*parent, line_edges, nodes_ancestors);
                let parent_ancestors = nodes_ancestors[*parent as usize].clone().unwrap();
                for ancestor in parent_ancestors{
                    ancestor_set.insert(ancestor);
                }
            }
            ancestor_set.insert(*parent);
        }
        nodes_ancestors[node as usize] = Some(ancestor_set.iter().map(|ancestor| *ancestor).collect());
    }
}
#[cfg(test)]
mod tests{
    use crate::p2194::Solution;
    #[test]
    pub fn test1(){
        let input1 = 8;
        let input2 = vec![vec![0,3],vec![0,4],vec![1,3],vec![2,4],vec![2,7],vec![3,5],vec![3,6],vec![3,7],vec![4,6]];
        let output = Solution::get_ancestors(input1, input2);
        assert_eq!(vec![vec![],vec![],vec![],vec![0,1],vec![0,2],vec![0,1,3],vec![0,1,2,3,4],vec![0,1,2,3]], output);
    }
    #[test]
    pub fn test2(){
        let input1 = 5;
        let input2 = vec![vec![0,1],vec![0,2],vec![0,3],vec![0,4],vec![1,2],vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![3,4]];
        let output = Solution::get_ancestors(input1, input2);
        assert_eq!(vec![vec![],vec![0],vec![0,1],vec![0,1,2],vec![0,1,2,3]], output);
    }
}