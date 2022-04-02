package main

import "fmt"
import "math/rand"
import "os"
import "bufio"

var key_egoism = "egoism"
var mindstate_goodmood = "good_mood"

const n_actors = 1000

var actors [n_actors]Actor

type Actor struct {
	money 		int
	traits 		map[string]float64
	mindstate 	map[string]float64
}

func play(player1 Actor, player2 Actor) (Actor, Actor){
	if(player1.mindstate[mindstate_goodmood] > player1.traits[key_egoism]) {
		player1.money = player1.money - 1
		player2.money = player2.money + 2
		player2.mindstate[mindstate_goodmood] = 0.5 + player2.mindstate[mindstate_goodmood] / 2
	}else{
		player2.mindstate[mindstate_goodmood] = player2.mindstate[mindstate_goodmood] / 2
	}

	return player1, player2
}

func game() {
	p1 := rand.Intn(len(actors))
	p2 := rand.Intn(len(actors))
	if (p1 != p2) {
		fmt.Println("Players playing: ", p1, p2)
		fmt.Println("Money before: ", actors[p1].money, actors[p2].money)
		pobj1, pobj2 := play(actors[p1], actors[p2])
		actors[p1] = pobj1
		actors[p2] = pobj2
		fmt.Println("Money after: ", actors[p1].money, actors[p2].money)
	}
}

func main() {

	for i := 0; i<n_actors; i++ {
		actors[i].money = 100
		actors[i].traits = make(map[string]float64)
		actors[i].mindstate = make(map[string]float64)

		actors[i].traits[key_egoism] = rand.Float64()
		actors[i].mindstate[mindstate_goodmood] = 0.5
	}

	for i := 0; i < 10000000; i++ {
		game()
	}
	
	for i := 0; i < n_actors; i++ {
		fmt.Println("Player ", i+1, " Egoism: ", actors[i].traits[key_egoism], "Money: ", actors[i].money)
	}
	
	f, _ := os.Create("data.sim")
	defer f.Close()

	w := bufio.NewWriter(f)
	defer w.Flush()

	fmt.Fprintf(w, "%10s %10s %10s\n", "# PlAyEr", "egoism", "money")
	for i := 0; i < n_actors; i++ {
		fmt.Fprintf(w, "%10d %10f %10d\n", i, actors[i].traits[key_egoism], actors[i].money)
	}
}
