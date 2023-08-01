#[derive(Debug)]
struct Voter<const N: usize> {
    prefrence: [i32; N],
}

impl<const N: usize> Voter<N> {
    pub fn new(prefrence: [i32; N]) -> Self {
        return Voter { prefrence };
    }

    pub fn choose_candidate(&self, candidates: &Vec<Self>) -> Option<(usize, i32)> {
        let mut closest_candidate: Option<(usize, i32)> = None;

        for (idx, candidate) in candidates.iter().enumerate() {
            let distance: i32 = zip(candidate.prefrence, self.prefrence)
                .map(|(a, b)| (a - b).abs().pow(2))
                .sum();

            if closest_candidate.is_none() || distance < closest_candidate.unwrap().1 {
                closest_candidate = Some((idx, distance));
            }
        }
        return closest_candidate;
    }
}

pub fn run_simulation() {
    
    // 1. Lage alle stemmegivere
    let mut voters: Vec<Voter<1>> = vec![];
    let mut candidates: Vec<Voter<1>> = vec![];

    for i in 0..100 {
        voters.push(Voter::new([i]))
    }
    candidates.clear();

    // 2. Sette opp kandidater
    candidates.push(Voter::new([1]));
    candidates.push(Voter::new([49]));
    candidates.push(Voter::new([50]));
    candidates.push(Voter::new([51]));
    candidates.push(Voter::new([95]));
    candidates.push(Voter::new([100]));

    // 3. Gjennomføre valget

    let voted_for: Vec<_> = voters
        .iter()
        .map(|v| v.choose_candidate(&candidates).unwrap().0)
        .collect();

    // 4. Finne resultatet
    let mut candidate_result: Vec<i32> = Vec::new();

    for _ in 0..candidates.len() {
        candidate_result.push(0);
    }

    for vote in voted_for {
        candidate_result[vote] += 1;
    }

    println!("{:#?}", candidate_result)
}