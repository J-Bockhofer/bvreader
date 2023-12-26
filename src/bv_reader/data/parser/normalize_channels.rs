


pub fn process_chanchangus(mut chanchungus: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    let mut cnt = 0;    
    for i in 0..chanchungus.len() {
        let chan_len = chanchungus[i].len();
        let sum: f32 = chanchungus[i].clone().iter().sum();
        let avg = sum / chan_len as f32;

        for j in 0..chan_len {
            chanchungus[i][j] = chanchungus[i][j] / avg;
        }

        println!("Chan-{},  Len: {},  Avg: {}", cnt, chan_len, avg);
        cnt += 1;
    }

    chanchungus
}