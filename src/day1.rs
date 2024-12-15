use std::fs;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

type List = Vec<i32>;
type PairList = (List,List);

pub fn day1() {
    let data = read_buf().unwrap();
    assert_eq!(data.0.len(),data.1.len());
    let sorted_lhs = merge_sort(&data.0);
    let sorted_rhs = merge_sort(&data.1);
    let sorted_data:PairList = (sorted_lhs,sorted_rhs);
    let sum = part1(&sorted_data);
    println!("{}",sum);

    let sim = part2(&sorted_data);
    println!("{}",sim);
}

fn part1(data:&PairList) -> i32 {

    let mut sum:i32 = 0;
    for ct in 0..data.0.len() {
        sum += (data.1[ct] - data.0[ct]).abs();
    }
    sum
}

fn part2(data:&PairList) -> i32 {
    let mut counted_values: HashMap<i32,i32> = HashMap::new();

    let mut sim = 0;
    let mut il = 0;
    let mut ir = 0;
    let rhs = data.1.to_vec();
    let lhs = data.0.to_vec();

    assert_eq!(rhs.len(),lhs.len());
    while il < lhs.len() {
        let val = lhs[il];
        if let Some(existing)  = counted_values.get(&val) {
            sim += val*existing;
        } else {
            while rhs[ir] < val {
                ir += 1;
            } 
            let mut ct: i32 = 0;
            while rhs[ir] == val {
                ct += 1;
                ir += 1;
            }
            sim += val*ct;
            counted_values.insert(val, ct);
        }
        il += 1;
    }


    sim

}

fn read_buf() -> io::Result<PairList> {
    let file = fs::File::open("day1.txt")?;
    let reader = BufReader::new(file);
    let mut data:PairList = (Vec::new(),Vec::new());
    for line in reader.lines() {
        let line: String = line?;
        let mut res = line.split_whitespace();
        let v1 = res.next().unwrap().to_string();
        let v2 = res.next().unwrap().to_string();
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

#[cfg(test)]
mod tests {
    use crate::day1::part1;
    use super::merge_sort;
    use super::part2;
    use super::List;

    #[test]
    fn test_mergesort() {
        let exp:Vec<i32> = vec![5,6,7,11,12,13];
        let arr:Vec<i32> = vec![12,11,13,5,6,7];
        let sarr = merge_sort(&arr);
        assert_eq!(arr.len(),sarr.len());
        let mut ct = 0;
        for val in sarr {
            assert_eq!(val,exp[ct]);
            ct += 1;
        }


    }

    #[test]
   fn test_pt1() {
    let exp = 11;
    let v1:List = vec![3,4,2,1,3,3];
    let v2:List = vec![4,3,5,3,9,3];

    let sum = part1(&(merge_sort(&v1),merge_sort(&v2)));
    assert_eq!(sum,exp);
   }

   #[test]
   fn test_pt2() {
    let exp = 31;
    let v1:List = vec![3,4,2,1,3,3];
    let v2:List = vec![4,3,5,3,9,3];
    let sim = part2(&(merge_sort(&v1),merge_sort(&v2)));
    assert_eq!(sim,exp);
   }
}