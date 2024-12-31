

fn swap(arr: &mut Vec<String>, lo: usize, hi: usize) {
    //unsafe { core::ptr::swap(arr[lo].as_mut_ptr(), arr[hi].as_mut_ptr()); }
	arr.swap(lo, hi);
}

fn wmerge(arr: &mut Vec<String>, mut lo1: usize, hi1: usize, mut lo2: usize, hi2: usize, mut w: usize) {
	while (lo1 < hi1) && (lo2 < hi2) {
		let mut lo_old = 0;
		if arr[lo1] <= arr[lo2] {
			lo_old = lo1;
			lo1 += 1;
		} else {
			lo_old = lo2;
			lo2 += 1;
		}
		let w_old = w;
		w += 1;
		swap(arr, w_old, lo_old)
	}
	while lo1 < hi1 {
		let w_old = w;
		w += 1;
		let lo_old = lo1;
		lo1 += 1;
		swap(arr, w_old, lo_old)
	}
	while lo2 < hi2 {
		let w_old = w;
		w += 1;
		let lo_old = lo2;
		lo2 += 1;
		swap(arr, w_old, lo_old)
	}
}

fn wsort(arr: &mut Vec<String>, lo: usize, hi: usize, w: usize) {
	if (hi - lo) > 1 {
		let m = (lo + hi) / 2;
		imsort(arr, lo, m);
		imsort(arr, m, hi);
		wmerge(arr, lo, m, m, hi, w)
	} else if lo != hi {
		swap(arr, lo, w)
	}
}

fn imsort(arr: &mut Vec<String>, lo: usize, hi: usize) {
	if (hi - lo) > 1 {
		let m = (lo + hi) / 2;
		let mut w = lo + hi - m;
		wsort(arr, lo, m, w);
		while (w - lo) > 2 {
			let n = w;
			w = (lo + n + 1) / 2;
			wsort(arr, w, n, lo);
			wmerge(arr, lo, lo + n - w, n, hi, w);
		}
        let mut i = w;
		while i > lo {
            let mut j = i;
			while (j < hi) && (arr[j] < arr[j - 1]) {
				swap(arr, j, j - 1);
                j += 1;
			}
            i -= 1;
		}
	}
}

fn permute(l: &mut[Vec<usize>], n: usize, m: usize, pos: usize) {
	if n == 0 {
		return
	}
	let mut size: usize = 1;
	for i in 0..(n-1) {// for i := 0; i < n - 1; i++ {
		size *= m;
	}
	for i in 0..m {//for i := 0; i < m; i++ {
		for j in 0..size{//for j := 0; j < size; j++ {
			l[i * size + j][pos] = 'z' as usize - i as usize;
		}
		permute(&mut l[(i * size)..], n - 1, m, pos + 1);
	}
}

fn gen_array(n: usize, m: usize, size: &mut usize) -> Vec<String> {
	*size = 1;
    for i in 0..n {
        *size *= m as usize
	}
    let mut l: Vec<Vec<usize>> = vec![vec![0; n]; *size];
    let t0 = std::time::Instant::now();
	permute(&mut l, n, m, 0);
    let t1 = std::time::Instant::now();
	print!("[info] permute: {} ns\n", (t1 - t0).as_nanos());
	let mut result = vec![String::new(); *size];
    //for i := 0; i < *size; i++ {
	for i in 0..*size {
    	// fmt.Println(string(l[i]));
		result[i] = l[i].iter().map(|usize| char::from_u32(*usize as u32).unwrap()).collect();
	}
	return result
}

fn verify_array(l: &Vec<String>) {
	//for i := 1; i < len(l); i++ {
	for i in 1..l.len() {
    	if l[i - 1] > l[i] {
			panic!("badness");
		}
	}
}

pub fn main() {
	let mut size = 0;
    let t0 = std::time::Instant::now();
	let mut l = gen_array(6, 18, &mut size);
    let t1 = std::time::Instant::now();
	imsort(&mut l, 0, size);
    let t2 = std::time::Instant::now();

	print!("[info] gen_array: {} ns\n", (t1 - t0).as_nanos());
	print!("[info] sort: {} ns\n", (t2 - t1).as_nanos());
	verify_array(&l);
}