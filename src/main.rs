use std::{collections::HashSet, f32::NEG_INFINITY};
use ndarray::{arr2, Array2, s, Array1};
use ndarray_stats::QuantileExt;

struct Node<'a> {
    // For decision node
    feature_index: u16,
    threshold: f32,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
    info_gain: f32,

    // For leaf Node
    value: i32
}

struct Tree<'a> {
    root: Option<&'a Node<'a>>,
    min_samples_split: u16,
    max_depth: u16
}

impl Tree<'static>{
    pub fn new(a: u16, b: u16) -> Tree<'static>{
        return Tree {
            root: None,
            min_samples_split: a, 
            max_depth: b
        }
    }

    pub fn build_tree(&self, dataset: Array2<i32>, curr_depth: u16) -> Node {
        let shape = dataset.shape();
        let slice_end = shape[0] - 1;
    
        let args = dataset.slice(s![0..slice_end, 1]);
        let datashape = args.shape();
        let num_cols = dataset.ncols();
        let Y = dataset.column(num_cols - 1).to_owned();
        let num_samples = datashape[0] as u16;
        let num_features = datashape[1] as u16;
    
        if num_samples >= self.min_samples_split && curr_depth <= self.max_depth {
            let best_split = self.get_best_split(dataset, num_samples, num_features);
            if best_split.info_gain > 0.0 {
                let left_tree = self.build_tree(best_split.dataset_left.to_owned(), curr_depth + 1);
                let right_tree = self.build_tree(best_split.dataset_right.to_owned(), curr_depth + 1);
    
                return Node {
                    feature_index: best_split.feature_index,
                    threshold: best_split.threshold as f32,
                    left: Some(&left_tree),
                    right: Some(&right_tree),
                    info_gain: best_split.info_gain,
                    value: 0,
                };
            } else {
                // let leaf_value = self.calc_leaf(Y);
                return Node {
                    feature_index: 0,
                    threshold: 0.0,
                    left: None,
                    right: None,
                    info_gain: best_split.info_gain,
                    value: 0,
                };
            }
        } else {
            // Handle the case when the conditions are not met
            return Node {
                feature_index: 0,
                threshold: 0.0,
                left: None,
                right: None,
                info_gain: 0.0,
                value: 0,
            };
        }
    }
    

    pub fn calc_leaf(&self, Y: Array1<i32>){
        Y.max();
    }

    pub fn get_best_split(&self, dataset: Array2<i32>, num_samples: u16, num_features:u16) -> Split{
        let feature_index = 0;
        let thresholds = 0.0;
        let info_gain = NEG_INFINITY;
        let mut best_split: Option<Split> = None;
        
        for feature_index in 0..num_features{
            let column_vector = dataset.column(feature_index as usize).to_owned();
            let possible_thresholds: HashSet<&i32> = column_vector.iter().collect();
            for threshold in possible_thresholds{
                let dataset_left = self.split_left(&dataset, feature_index as usize, *threshold);
                let dataset_right = self.split_right(&dataset, feature_index as usize, *threshold);

                if dataset_left.len() > 0 && dataset_right.len() >0{
                    let num_cols = dataset.ncols();
                    let y = dataset.column(num_cols - 1).to_owned();
                    let curr_info_gain = self.information_gain(y, dataset_left, dataset_right, true);
                    if curr_info_gain > info_gain{
                        best_split = Some(Split{
                            feature_index: feature_index,
                            threshold: *threshold,
                            dataset_left: dataset_left,
                            dataset_right: dataset_right,
                            info_gain: curr_info_gain,
                        })
                    }
                }
            }

        }
        return best_split.unwrap();
    }

    pub fn information_gain(&self, parent: Array1<i32>, left: Array1<i32>, right: Array1<i32>, gini: bool) -> f32{
        let weight_l = left.len() as f32 / parent.len() as f32;
        let weight_r = right.len() as f32 / parent.len() as f32;
        let mut gain = 0.0;
        if gini == true{
            gain = self.gini_index(parent) - (weight_l*self.gini_index(left) + weight_r*self.gini_index(right));
        }
        else {
            //use entropy
        }
        return gain;
    }

    pub fn gini_index(self, y: Array1<i32>) -> f32{
        return 0.0;
    }
    pub fn split_left(&self, dataset: &Array2<i32>, feature_index: usize, threshold: i32) -> Array1<i32> {
        let thresh = threshold as i32;
        let column = dataset.column(feature_index).to_owned().iter().cloned();
        let dataset_left = Array1::from_vec(column.filter(|&x| x <= thresh).collect());
        return dataset_left;
    }
    
    pub fn split_right(&self, dataset: &Array2<i32>, feature_index: usize, threshold: i32) -> Array1<i32>{
        let thresh = threshold as i32;
        let column = dataset.column(feature_index).to_owned().iter().cloned();
        let dataset_right = Array1::from_vec(column.filter(|&x| x > thresh).collect());
        return dataset_right;
    }
}

struct Split{
    feature_index: u16,
    threshold: i32,
    dataset_left: Array1<i32>,
    dataset_right: Array1<i32>,
    info_gain: f32
}
fn main() {
    let a = arr2(&[
        [1,2,3],
        [4,5,6]
    ]);

    let b = arr2(&[
        [7,8,9],
        [10,11,12]
    ]);

    println!("{}", a.dot(&b.t()));
}
