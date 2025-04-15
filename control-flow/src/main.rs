fn main() {
    let  weather = "sunny";

    if weather == "sunny" {
        println!("Crabby will cross the river by smiling!");
    }else if weather == "rainy" {
       println!("Crabby will build a bridge to stay dry!");
    }else {
        println!("Crabby will wait for better weather!");
    }

    let enemy = "goblin";
    match enemy {
        "goblin" => println!("Crabby will fight the goblin! 🤺"),
        "troll" => println!("Crabby will outsmart the troll!"),
        "dragon" => println!("Crabby will run away from the dragon! 🐉"),
        _ => println!("Crabby will find a way to escape!"),
    }


    let mut wood = 0;
    // create loop to help Crabby collect wood!
    loop {
        if wood >= 10 {
            println!("Crabby has enough wood to build a bridge!");
            break;
        } else {
            println!("Crabby got {} woods!", wood);
            // Simulate collecting wood
            wood += 1;
        }
    }

}
