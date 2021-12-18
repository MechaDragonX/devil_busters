mod field;
mod monster;
mod resistance;
mod terrain;

use field::Field;
use monster::Monster;
use resistance::Resistance;
use terrain::Terrain;

fn main() {
    // let mut jack_frost: Monster = Monster::new("Jack Frost", "Fairy", 7, 78, 45);
    // jack_frost.dist_stats(14, 15, 29, 21, 21);
    // jack_frost.dist_element_res(Resistance::Normal, Resistance::Weak, Resistance::Drain, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal);
    // jack_frost.dist_ailment_res(Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Weak, Resistance::Normal, Resistance::Weak);
    
    // let mut slime: Monster = Monster::new("Slime", "Slime", 1, 7, 0);
    // slime.dist_stats(10, 2, 0, 4, 2);
    // slime.dist_element_res(Resistance::Normal, Resistance::Weak, Resistance::Weak, Resistance::Weak, Resistance::Weak, Resistance::Weak, Resistance::Weak);
    // slime.dist_ailment_res(Resistance::Weak, Resistance::Weak, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal);

    // let mut cactuar: Monster = Monster::new("Cactuar", "Cactuar", 7, 80, 1);
    // cactuar.dist_stats(23, 22, 50, 20, 15);
    // cactuar.dist_element_res(Resistance::Normal, Resistance::Weak, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal, Resistance::Normal);
    // cactuar.dist_ailment_res(Resistance::Normal, Resistance::Strong, Resistance::Weak, Resistance::Strong, Resistance::Strong, Resistance::Strong, Resistance::Normal, Resistance::Normal, Resistance::Normal);

    // let mut pikachu: Monster = Monster::new("Pikachu", "Mouse", 5, 30, 40);
    // pikachu.dist_stats(35, 15, 25, 70, 20);
    // pikachu.dist_element_res(Resistance::Weak, Resistance::Normal, Resistance::Normal, Resistance::Drain, Resistance::Strong, Resistance::Normal, Resistance::Normal);
    // pikachu.dist_ailment_res(Resistance::Normal, Resistance::Strong, Resistance::Normal, Resistance::Strong, Resistance::Strong, Resistance::Weak, Resistance::Normal, Resistance::Weak, Resistance::Normal);

    // println!("{}", jack_frost);
    // println!("{}", slime);
    // println!("{}", cactuar);
    // println!("{}", pikachu);

    let mut coburg: Field = Field::new(5, 5);
    coburg.area[0][0] = Terrain::Water;
    coburg.area[0][1] = Terrain::Water;
    coburg.area[0][2] = Terrain::Water;
    coburg.area[0][3] = Terrain::Mountains;
    coburg.area[0][4] = Terrain::Mountains;
    coburg.area[1][0] = Terrain::Water;
    coburg.area[1][1] = Terrain::Water;
    coburg.area[1][2] = Terrain::Water;
    coburg.area[1][3] = Terrain::Mountains;
    coburg.area[1][4] = Terrain::Mountains;
    coburg.area[2][0] = Terrain::Water;
    coburg.area[2][1] = Terrain::Water;
    coburg.area[2][2] = Terrain::Water;
    coburg.area[2][3] = Terrain::Mountains;
    coburg.area[2][4] = Terrain::Mountains;
    coburg.area[3][0] = Terrain::Land;
    coburg.area[3][1] = Terrain::Land;
    coburg.area[3][2] = Terrain::Land;
    coburg.area[3][3] = Terrain::Mountains;
    coburg.area[3][4] = Terrain::Mountains;
    coburg.area[4][0] = Terrain::Land;
    coburg.area[4][1] = Terrain::Land;
    coburg.area[4][2] = Terrain::Land;
    coburg.area[4][3] = Terrain::Land;
    coburg.area[4][4] = Terrain::Land;
    println!("{}", coburg);
}
