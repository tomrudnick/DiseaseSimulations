use threadpool::ThreadPool;
use std::sync::mpsc;

mod sim1d;
mod sim2d;


//step size is 0.01
fn run_simulation(simulations: i32, t_max: f64, lambda_range: (f64, f64)) -> Vec<(f64, f64, f64)>{
    let mut simulation_results = Vec::new();
    let n_workers = 12;
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(n_workers);

    let lower_bound = (lambda_range.0 * 100.0) as i32;
    let upper_bound = (lambda_range.1 * 100.0) as i32;

    let simulations = simulations;
    for lambda in lower_bound..=upper_bound {
        let tx = tx.clone();

        pool.execute(move || {
            let lambda = lambda as f64 / 100.0;
            let mut success = 0;
            let mut end_nodes_sum = 0;  
            println!("Lambda: {}", lambda);
            for _i in 0..simulations {
                let mut sim = sim2d::Simulation::new(lambda, 1.0);
                let result = sim.run(t_max);
                if result {
                    success += 1;
                }
                end_nodes_sum += sim.get_number_of_infected_nodes();
            }
            let end_nodes_avg = end_nodes_sum as f64 / simulations as f64;
            let success_avg = success as f64 / simulations as f64;
            tx.send((lambda, success_avg, end_nodes_avg)).unwrap();
        });
    }
    pool.join();
    drop(tx);
    for (lambda, success, end_nodes_average) in rx {
        simulation_results.push((lambda, success, end_nodes_average));
    }


    simulation_results.sort_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());

    return simulation_results;
}






fn print_results(results: &Vec<(f64, f64, f64)>) {
    println!("Lambda, Success, End Nodes Average");
    for (lambda, success, end_nodes_average) in results {
        println!("lambda {}, disease died percentage {}, average of nodes at end of simulation {}", lambda, success, end_nodes_average);
    }
}

fn print_results_to_csv_file(results: &Vec<(f64, f64, f64)>, file_name: &str) {
    let mut wtr = csv::Writer::from_path(file_name).unwrap();
    wtr.write_record(&["Lambda", "Success", "End Nodes Average"]).unwrap();
    for (lambda, success, end_nodes_average) in results {
        wtr.write_record(&[lambda.to_string(), success.to_string(), end_nodes_average.to_string()]).unwrap();
    }
    wtr.flush().unwrap();
}

fn main() {
    //let mut sim = Simulation::new(2.0);
    //sim.run(1000000f64);
    let result = run_simulation(100, 1000f64, (0.8, 1.6));
    print_results(&result);
    //print_results_to_csv_file(&result, "results.csv");
    //let mut sim = sim2d::Simulation::new(1.1, 0.01);
    //sim.run(1000000f64);

}
