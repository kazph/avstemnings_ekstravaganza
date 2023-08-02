use std::iter::zip;

#[derive(Debug)]
struct Voter<const N: usize> {
    prefrence: [i32; N],
}

#[derive(Debug, serde::Deserialize)]
pub struct ElectionConfig {
    id: String,
    name: String,
    width_of_spectre: i32,
    voter_distribution: String,
    candidate_distribution: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ElectionResult {
    winners_prefrence: i32,
    number_of_voters: i32,
    prefrence_distance: i32,
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

fn str_to_ints(text: String) -> Vec<i32> {
    return text
        .split(" ")
        .into_iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();
}

fn max_element_index(list: &Vec<i32>) -> (usize, i32) {
    let mut max = (0, i32::MIN);

    for (idx, elem) in list.iter().enumerate() {
        if *elem > max.1 {
            max = (idx, *elem)
        }
    }

    return max;
}

pub fn run_simulation(config: ElectionConfig) -> ElectionResult {
    // 1. Lage alle stemmegivere
    let mut voters: Vec<Voter<1>> = vec![];
    let mut candidates: Vec<Voter<1>> = vec![];

    let voter_dist = str_to_ints(config.voter_distribution);
    let candidate_dist = str_to_ints(config.candidate_distribution);

    for (prefrence, num_voters) in voter_dist.iter().enumerate() {
        for _ in 0..*num_voters {
            voters.push(Voter::new([prefrence.try_into().unwrap()]))
        }
    }

    // 2. Sette opp kandidater
    for (prefrence, num_candidates) in candidate_dist.iter().enumerate() {
        for _ in 0..*num_candidates {
            candidates.push(Voter::new([prefrence.try_into().unwrap()]))
        }
    }

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

    let winner = max_element_index(&candidate_result);

    println!("{:#?}", candidate_result);

    ElectionResult {
        winners_prefrence: candidates[winner.0].prefrence[0],
        number_of_voters: voters.len().try_into().unwrap(),
        prefrence_distance: voters
            .iter()
            .map(|v| v.prefrence[0])
            .map(|p| p.pow(2))
            .sum(),
    }
}
