package main

import "fmt"
import "math/rand"

var key_egoism = "egoism"

var actors [20]Actor

type Actor struct {
	money 		int
	traits 		map[string]float64
	mindstate 	map[string]float64
}

func play(player1 Actor, player2 Actor) (Actor, Actor){
	if(player1.traits[key_egoism] < 0.5) {
		player1.money = player1.money - 1
		player2.money = player2.money + 2
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

	for i := 0; i<20; i++ {
		actors[i].money = 100
		actors[i].traits = make(map[string]float64)
		actors[i].mindstate = make(map[string]float64)

		actors[i].traits[key_egoism] = rand.Float64()
	}

	for i := 0; i < 1000; i++ {
		game()
	}
	
	for i := 0; i < 20; i++ {
		fmt.Println("Player ", i+1, " Egoism: ", actors[i].traits[key_egoism], "Money: ", actors[i].money)
	}
}
