use blake3::Hasher;
use noise::{NoiseFn, OpenSimplex};

const FINE_SCALE: f64 = 0.5;
const FINE_STRENGTH: f64 = 0.1;
const SEA_LEVEL: f64 = -0.1;

#[derive(Debug)]
pub struct WorldGenerator {
    broad: Layer,
    fine: Layer,
}

impl WorldGenerator {
    pub fn new(scale: f64, seed: u32) -> Self {
        let broad = OpenSimplex::new(generate_hash(seed, b"broad"));
        let fine = OpenSimplex::new(generate_hash(seed, b"fine"));

        WorldGenerator {
            broad: Layer {
                scale,
                noise: broad,
            },
            fine: Layer {
                scale: scale * FINE_SCALE,
                noise: fine,
            },
        }
    }

    pub fn elevation(&self, x: f64, y: f64) -> f64 {
        let mut result = self
            .broad
            .noise
            .get([x / self.broad.scale, y / self.broad.scale]);

        result += self
            .fine
            .noise
            .get([x / self.fine.scale, y / self.fine.scale])
            * FINE_STRENGTH;

        result
    }

    pub fn terrain(&self, x: f64, y: f64) -> Terrain {
        if self.elevation(x, y) < SEA_LEVEL {
            Terrain::Ocean
        } else {
            Terrain::Land
        }
    }
}

#[derive(Debug)]
struct Layer {
    noise: OpenSimplex,
    scale: f64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Terrain {
    Land,
    Ocean,
}

fn generate_hash(seed: u32, salt: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(salt);
    hasher.update(&seed.to_le_bytes());

    let hash = hasher.finalize();
    // SAFETY: BLAKE3 always returns 32 bytes, so this slice exists
    // and has the four bytes required.
    let four: [u8; 4] = hash.as_bytes()[..4].try_into().unwrap();

    u32::from_le_bytes(four)
}
