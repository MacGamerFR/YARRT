use super::Vec3;
use rand::Rng;

pub struct Perlin {
    point_count: usize,
    random_floats: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new(point_count: usize) -> Perlin {
        let mut ranfloat = Vec::with_capacity(point_count);
        for _i in 0..point_count {
            ranfloat.push(rand::thread_rng().gen());
        }

        Perlin {
            point_count,
            random_floats: ranfloat,
            perm_x: Perlin::generate_permutations(point_count, point_count),
            perm_y: Perlin::generate_permutations(point_count, point_count),
            perm_z: Perlin::generate_permutations(point_count, point_count),
        }
    }

    pub fn noise(&self, point: &Vec3) -> f64 {
        let i = point.x.floor();
        let j = point.y.floor();
        let k = point.z.floor();

        let u = point.x - i;
        let v = point.y - j;
        let w = point.z - k;

        let mut c = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_floats[self.perm_x
                        [((i as isize + di as isize) & 255) as usize]
                        ^ self.perm_y[((j as isize + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k as isize + dk as isize) & 255) as usize]];
                }
            }
        }

        trilinear_interpolation(u, v, w, &c)
    }

    fn generate_permutations(capacity: usize, point_count: usize) -> Vec<usize> {
        let mut values = Vec::with_capacity(capacity);
        for i in 0..point_count {
            values.push(i);
        }
        Perlin::permute(&mut values[..], point_count);
        values
    }

    /// Executes `count` permutations on `ptr` randomly
    fn permute(ptr: &mut [usize], count: usize) {
        for i in (1..count - 1).rev() {
            let target: usize = rand::thread_rng().gen_range(0, i);
            ptr.swap(i, target);
        }
    }
}

pub fn trilinear_interpolation(u: f64, v: f64, w: f64, c: &[[[f64; 2]; 2]; 2]) -> f64 {
    let mut acc = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i_f = i as f64;
                let j_f = j as f64;
                let k_f = k as f64;
                acc += (i_f * u + (1.0 - i_f) * (1.0 - u))
                    * (j_f * v + (1.0 - j_f) * (1.0 - v))
                    * (k_f * w + (1.0 - k_f) * (1.0 - w))
                    * c[i][j][k];
            }
        }
    }

    acc
}
