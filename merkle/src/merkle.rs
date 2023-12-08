

struct MerkleTree {
    values: Vec<i32>,
    hashes: Vec<i32>,
}
impl MerkleTree {
    fn new() -> Self {
        MerkleTree {
            values: Vec::new(),
            hashes: Vec::new(),
        }
    }

    fn add(&mut self, data: i32){
        self.values.push(data);
        self.update_hashes();
    }

    fn update_hashes(&mut self) {
        self.hashes.clear();
        for(i, value) in self.values.iter().enumerate() {
            // if(i != self.values.len() - 1) {
                self.hashes.push(*value);
            // }
            
        }
        let mut k = 0;
        if self.hashes.len()==0 {
            return
        }
        for i in self.hashes.len()..2*self.hashes.len()-1 {
            self.hashes.push(self.hashes[k*2] + self.hashes[(k*2)+1]);
            k += 1;
        }

        println!("hashes {:?}", self.hashes);
    }

    fn get_proof_of_leafi(&self, i: i32) -> Vec<Vec<i32>> {
        let mut proof = Vec::<Vec<i32>>::new();
        
        println!("i {:?}", (self.hashes.len().ilog2() as f32 + 0.).floor() as i32);

        let mut k = i as usize;

        for j in 0..self.hashes.len()/(self.hashes.len().ilog2() as f32 + 0.).floor() as usize {
            println!("level {:?}", j);
            if(k%2 == 0 ){
                if(k+1 >= self.hashes.len()) {
                    break;
                }
                proof.push(vec![self.hashes[k+1], 0]);
                k = self.values.len() + (k/2);
            } else {
                if(k-1 >= self.hashes.len()) {
                    break;
                }
                proof.push(vec![self.hashes[k-1], 1]);
                k = self.values.len() + (k/2);
            }
        }

        // let mut j = i
        // while j<self.hashes.len() {
            
        // }

        println!("proof {:?}", proof);
        proof         
    }
}

pub fn merkle_tree() {

    let mut tree = MerkleTree::new();
    tree.add(1);
    tree.add(2);
    tree.add(3);
    tree.add(4);
    tree.add(5);

    tree.get_proof_of_leafi(5);

}