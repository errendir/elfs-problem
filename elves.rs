// Run: cargo run --release --bin elves
// Debug run (with extra output): cargo run --bin elves

struct ElfSim {
    size: usize,
}

impl ElfSim {
    fn new(size: usize) -> ElfSim {
        ElfSim { size }
    }

    // The size of the range from start to end
    // If end > start it's simple
    // If start > end, we loop around the array
    fn range_size(&self, start: usize, end: usize) -> usize {
        if end >= start {
            return end - start;
        } else {
            return (end + self.size) - start;
        }
    }

    fn simulate_elves(&self) {
        let mut items: Vec<i64> = vec![0; self.size];

        for i in 0..items.len() {
            items[i] = (i + 1) as i64;
        }

        // pointer 1: the elf that is currently holding the gun
        let mut origin_i: usize = 0;

        // pointer 2: the first index of the empty segment
        let mut empty_i: usize = self.size / 2;

        // pointer 3: the elf currently being shot
        let mut target_i: usize = self.size / 2;

        let mut last_elf = 0;

        // invariant: if you walk forward (with looping) from origin_i you encounter
        // alive elves, then the empty_i pointer, starting at which
        // there are only dead elves until the target_i pointer
        // after which there are alive elves until you reach the origin_i pointer back
        // (we loop)

        let mut remaining_elves = self.size;

        while remaining_elves > 1 {
            // Figure out if the sides are balanced
            let left_side_size =
                self.range_size(origin_i, target_i) - self.range_size(empty_i, target_i);
            let right_side_size = self.range_size(target_i, origin_i);

            if !(left_side_size == right_side_size || left_side_size + 1 == right_side_size) {
                // Rewrite the elf at the target_i to the empty_i
                items[empty_i] = items[target_i];
                empty_i = (empty_i + 1) % self.size;

                if cfg!(debug_assertions) {
                    items[target_i] = -1;
                }
                target_i = (target_i + 1) % self.size;
            }

            last_elf = items[origin_i];

            // Erase the elf at the target_i
            if cfg!(debug_assertions) {
                println!("VECTOR {:?}", items);
                println!("ELF SHOOTING {} -> {}", items[origin_i], items[target_i]);
                items[target_i] = -1;
            }

            target_i = (target_i + 1) % self.size;
            origin_i = (origin_i + 1) % self.size;

            remaining_elves -= 1;

            if remaining_elves % 10_000_000 == 0 {
                println!("PROGRESS, REMAINING {}", remaining_elves);
            }
        }

        println!("RING SIZE {}, REMAINING ELF {}", self.size, last_elf)
    }
}

fn main() {
    // let sim = ElfSim::new(5);
    let sim = ElfSim::new(3012210);
    sim.simulate_elves();
}
