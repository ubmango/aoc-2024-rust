use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

type List     = Vec<i32>;
type PairList = (List,List);
  
const FILEPTH:&str = "./src/day1/day1.txt";

pub fn day1() -> (i32,i32) {
    // Read File
    let data:PairList   = read_datafile(FILEPTH).unwrap();

    // Sort Lists In Ascending Order
    let sorted_lhs:List = merge_sort(&data.0);
    let sorted_rhs:List = merge_sort(&data.1);
    let sorted_data:PairList = (sorted_lhs,sorted_rhs);

    // Part 1 Solution
    let sum:i32 = part1(&sorted_data);

    // Part 2 Solution
    let sim:i32 = part2(&sorted_data);

    (sum,sim)
}

/////////
// Part 1
/////////
fn part1(data:&PairList) -> i32 {
    let mut sum:i32 = 0;
    for ct in 0..data.0.len() {
        sum += (data.1[ct] - data.0[ct]).abs();
    }
    sum
}

/////////////////////////////
// Part 2
// NOTE: Requires sorted data
///////////////////////////// 
fn part2(sorted_data:&PairList) -> i32 {
    // Initialization
    let mut sim:i32  = 0;
    let mut il:usize = 0;
    let mut ir:usize = 0;

    // Map of values whose frequencies have already been computed
    let mut counted_values: HashMap<i32,i32> = HashMap::new();

    // Unpack Data
    let rhs:List = sorted_data.1.to_vec();
    let lhs:List = sorted_data.0.to_vec();
    assert_eq!(rhs.len(),lhs.len());

    // Check every value in lhs dataset
    while il < lhs.len() {
        let target:i32 = lhs[il]; // value to compute frequency of 

        // Check if value has already had its frequency calculated
        if let Some(frequency) = counted_values.get(&target) {
            sim += target*frequency; // compute simularity from existing value
        }
        else {
            let frequency:i32; // Compute frequency of target in the rhs dataset
            (frequency,ir) = sorted_frequency(&rhs,target,ir);

            // Compute simularity
            sim += target*frequency;

            // Store target and frequency in hashmap
            counted_values.insert(target, frequency);
        }
        il += 1;
    }
    sim
}

fn read_datafile(fpth:&str) -> io::Result<PairList> {
    let file:File = File::open(fpth)?;
    let reader:BufReader<File> = BufReader::new(file);
    let mut data:PairList = (Vec::new(),Vec::new());
    for line in reader.lines() {
        let line: String = line?;
        let mut res = line.split_whitespace();
        let v1:String = res.next().unwrap().to_string();
        let v2:String = res.next().unwrap().to_string();
        let i1:i32 = v1.parse().unwrap();
        let i2:i32 = v2.parse().unwrap();
        data.0.push(i1);
        data.1.push(i2);
    }
    Ok(data)
}

fn merge_sort(data:&List) ->  List {
    if data.len() < 2 {
        return data.to_vec();
    } else {
        let mid: usize = data.len()/2;
        let lhs = merge_sort(&data[..mid].to_vec());
        let rhs = merge_sort(&data[mid..].to_vec());
        let res = merge(&lhs,&rhs);
        res
    }
}

fn merge(lhs:&List,rhs:&List) -> List {
    let mut i = 0;
    let mut j = 0;
    let mut merged:List = Vec::new();

    while i < lhs.len() && j < rhs.len() {
        if lhs[i] < rhs[j] {
            merged.push(lhs[i]);
            i += 1;
        }
        else {
            merged.push(rhs[j]);
            j+=1;
        }
    }

    if i < lhs.len() {
        while i < lhs.len() {
            merged.push(lhs[i]);
            i+=1;
        }
    }

    if j < rhs.len() {
        while j < rhs.len() {
            merged.push(rhs[j]);
            j+=1;
        }
    }
    merged

}

fn sorted_frequency(data:&List,value:i32,min_index:usize) -> (i32,usize) {
    let mut im = min_index;
    while data[im] < value {
        im += 1;
    } 

    let mut frequency:i32 = 0;
    while data[im] == value {
        frequency += 1;
        im += 1;
    }
    (frequency,im)
}


/// Testing
#[cfg(test)]
mod tests {
    use super::merge_sort;
    use super::part1;
    use super::part2;
    use super::List;

    // Test sorting algorithm
    #[test]
    fn test_mergesort() {
        let exp:List  = vec![5,6,7,11,12,13];
        let arr:List  = vec![12,11,13,5,6,7];
        let sarr:List = merge_sort(&arr);
        assert_eq!(arr.len(),sarr.len());
        let mut ct:usize = 0;
        for val in sarr {
            assert_eq!(val,exp[ct]);
            ct += 1;
        }
    }

    // Part 1 Test Case
    #[test]
   fn test_pt1() {
    let exp:i32 = 11;
    let v1:List = vec![3,4,2,1,3,3];
    let v2:List = vec![4,3,5,3,9,3];
    let sum:i32 = part1(&(merge_sort(&v1),merge_sort(&v2)));
    assert_eq!(sum,exp);
   }

   // Part 2 Test Case
   #[test]
   fn test_pt2() {
    let exp:i32 = 31;
    let v1:List = vec![3,4,2,1,3,3];
    let v2:List = vec![4,3,5,3,9,3];
    let sim:i32 = part2(&(merge_sort(&v1),merge_sort(&v2)));
    assert_eq!(sim,exp);
   }
}