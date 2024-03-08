// radix_sort.rs

fn radix_sort(nums: &mut [usize]) {
    if nums.len() < 2 {
        return;
    }

    // 找到最大的数，它的位最多
    let max_num = match nums.iter().max() {
        Some(&x) => x,
        None => return,
    };

    // 找最接近且 >= nums 长度的 2 的次幂值作为桶大小，如:
    // 最接近且 >= 10 的 2 的次幂值是 2^4 = 16
    // 最接近且 >= 17 的 2 的次幂值是 2^5 = 32
    let mut radix = nums.len().next_power_of_two();

    // digit 代表小于某个位对应桶的所有数
    // 个、十、百、千分别在 1、2、3、4 位
    // 起始从个位开始，所以是 1
    let mut digit = 1;
    // radix = 11;
    while digit <= max_num {
        // 闭包函数：计算数据在桶中的位置
        let index_of = |x| x / digit % radix;

        // 计数器
        let mut counter = vec![0; radix];
        for &x in nums.iter() {
            counter[index_of(x)] += 1;
        }

        println!("counter, first iterate:{:?}", counter);
        for i in 1..radix {
            counter[i] += counter[i - 1];
        }

        println!("counter, radix iterate add:{:?}", counter);

        // 排序
        for &x in nums.to_owned().iter().rev() {
            counter[index_of(x)] -= 1;
            nums[counter[index_of(x)]] = x;
        }

        println!("digit:{},radix:{},counter:{:?}", digit, radix, counter);
        println!("digit:{},radix:{},nums:{:?}", digit, radix, nums);

        // 跨越桶
        digit *= radix;
    }
}

fn main() {
    let mut nums = [54, 32, 119,99, 666, 18, 75,333, 31, 918,43, 999,56,999,21, 22];
    println!("unsorted nums: {:?}", nums);
    radix_sort(&mut nums);
    println!("sorted nums: {:?}", nums);
}
