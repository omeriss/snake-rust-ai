pub mod train_network {
    const POPULATION_SIZE: usize = 70;
    const MUTATION_PERECENT: f64 = 20.;
    const AVERAGE_AMOUNT: usize = 10;

    use piston::UpdateArgs;

    use crate::{
        ai::{ai_game::ai_game::AiGame, neural_network::neural_network::NeuralNetwork},
        constants::constants::TIME_BETWEEN_MOVES,
    };

    pub fn run_game(ai_game: &mut AiGame) {
        let mut last_score = ai_game.get_game().get_score();
        let mut turns_from_last_score = 0;

        while ai_game.get_game().is_alive()
            && turns_from_last_score < 150
            && ai_game.get_game().get_turns() < 2500
        {
            let update_args: UpdateArgs = UpdateArgs {
                dt: TIME_BETWEEN_MOVES,
            };
            ai_game.update(&update_args);

            if last_score != ai_game.get_game().get_score() {
                last_score = ai_game.get_game().get_score();
                turns_from_last_score = 0;
            } else {
                turns_from_last_score += 1;
            }
        }
    }

    pub fn get_average_fitness(ai_game: &AiGame, amount: usize) -> f64 {
        let mut total_fitness = 0.;

        for _ in 0..amount {
            let mut game = ai_game.clone();
            run_game(&mut game);
            total_fitness += game.calc_fitness();
        }

        total_fitness / amount as f64
    }

    pub fn train_network(save_folder: &str, upload_file: &str) {
        let mut population: Vec<AiGame> = Vec::new();

        if upload_file != "" {
            // todo: move shape to file
            let base_nn = NeuralNetwork::new_from_file(vec![24, 40, 40, 4], upload_file);

            for _ in 0..POPULATION_SIZE {
                population.push(AiGame::new_from(None, &base_nn));
            }
        } else {
            for _ in 0..POPULATION_SIZE {
                population.push(AiGame::new(None));
            }
        }

        let mut gen = 0;

        let mut best_fintess = 0.;
        let mut best_of_all = population[0].clone();

        loop {
            for game in &mut population {
                run_game(game);
            }

            population.sort_by(|a, b| b.calc_fitness().total_cmp(&a.calc_fitness()));

            let average_fitness = get_average_fitness(&population[0], AVERAGE_AMOUNT);

            if best_fintess < average_fitness {
                best_fintess = average_fitness;
                best_of_all = population[0].clone();

                best_of_all
                    .get_neural_network()
                    .write_to_file(&format!("{}/best.bin", save_folder));
            }

            population[0]
                .get_neural_network()
                .write_to_file(&format!("{}/best_of_gen_{}.bin", save_folder, gen));

            println!(
                " gen {} best fintess {} with score {} in {} turns and average {}",
                gen,
                population[0].calc_fitness(),
                population[0].get_game().get_score(),
                population[0].get_game().get_turns(),
                average_fitness
            );

            // create the next generation
            let mut new_population: Vec<AiGame> = Vec::new();

            new_population.push(population[0].clone());
            new_population.push(best_of_all.clone());

            for _ in 2..10 {
                new_population.push(AiGame::new(None));
            }

            for _ in 10..POPULATION_SIZE {
                new_population.push(AiGame::new_from(None, &population[0].get_neural_network()));
                let last_element = new_population.len() - 1;
                new_population[last_element].mutate(MUTATION_PERECENT);
            }

            population = new_population;

            gen += 1;
        }
    }
}
