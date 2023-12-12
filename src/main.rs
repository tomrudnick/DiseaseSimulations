use std::fmt::Display;
use crate::simulation::sim::*;
use threadpool::ThreadPool;
use std::sync::mpsc;

mod simulation;

use simulation::*;

struct SimulationResult {
    lambda: f64,
    success_average: f64,
    end_nodes_average: f64,
}

struct SimulationResultAlpha {
    lambda: f64,
    alpha: f64,
    success_average: f64,
    end_nodes_average: f64,
}

trait WriteToCsv {
    fn write_to_csv(&self, wtr: &mut csv::Writer<std::fs::File>);
    fn write_header_to_csv(wtr: &mut csv::Writer<std::fs::File>);
}
impl Display for SimulationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "lambda {}, disease died percentage {}, average of nodes at end of simulation {}",
               self.lambda,
               self.success_average,
               self.end_nodes_average)
    }
}

impl Display for SimulationResultAlpha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "lambda {}, alpha {}, disease died percentage {}, average of nodes at end of simulation {}",
               self.lambda,
               self.alpha,
               self.success_average,
               self.end_nodes_average)
    }
}

impl WriteToCsv for SimulationResultAlpha {
    fn write_to_csv(&self, wtr: &mut csv::Writer<std::fs::File>) {
        wtr.write_record(&[
            self.lambda.to_string(),
            self.alpha.to_string(),
            self.success_average.to_string(),
            self.end_nodes_average.to_string()]).unwrap();
    }
    fn write_header_to_csv(wtr: &mut csv::Writer<std::fs::File>) {
        wtr.write_record(&["Lambda", "Alpha", "Disease died average", "End Nodes Average"]).unwrap();
    }
}

impl WriteToCsv for SimulationResult {
    fn write_to_csv(&self, wtr: &mut csv::Writer<std::fs::File>) {
        wtr.write_record(&[
            self.lambda.to_string(),
            self.success_average.to_string(),
            self.end_nodes_average.to_string()]).unwrap();
    }
    fn write_header_to_csv(wtr: &mut csv::Writer<std::fs::File>) {
        wtr.write_record(&["Lambda", "Disease died average", "End Nodes Average"]).unwrap();
    }
}



//step size is 0.01
fn run_simulation(simulations: i32, t_max: f64, lambda_range: (f64, f64)) -> Vec<SimulationResult>{
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
                let mut sim = sim1d::Simulation::new(lambda);
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
        simulation_results.push(SimulationResult{lambda, success_average: success, end_nodes_average});
    }


    simulation_results.sort_by(|a, b| a.lambda.partial_cmp(&b.lambda).unwrap());

    return simulation_results;
}


fn run_simulation_alpha(simulations: i32,
                        t_max: f64,
                        lambda_range: (f64, f64),
                        alpha_range: (f64, f64),
                        sim_alpha_type: SimAlphaType
                        ) -> Vec<SimulationResultAlpha>{
    let mut simulation_results = Vec::new();
    let n_workers = 12;
    let (tx, rx) = mpsc::channel();
    let pool = ThreadPool::new(n_workers);

    let lower_bound = (lambda_range.0 * 100.0) as i32;
    let upper_bound = (lambda_range.1 * 100.0) as i32;

    let lower_bound_alpha = (alpha_range.0 * 10.0) as i32;
    let upper_bound_alpha = (alpha_range.1 * 10.0) as i32;

    let simulations = simulations;
    for lambda in lower_bound..=upper_bound {
        let tx = tx.clone();
        for alpha in lower_bound_alpha..=upper_bound_alpha {
            let tx = tx.clone();
            pool.execute(move || {
                let lambda = lambda as f64 / 100.0;
                let alpha = alpha as f64 / 10.0;
                let mut success = 0;
                let mut end_nodes_sum = 0;
                println!("Lambda: {}, Alpha: {}", lambda, alpha);
                for _i in 0..simulations {
                    let mut sim = create_sim(sim_alpha_type, lambda, alpha);
                    let result = sim.run(t_max);
                    if result {
                        success += 1;
                    }
                    end_nodes_sum += sim.get_number_of_infected_nodes();
                }
                let end_nodes_avg = end_nodes_sum as f64 / simulations as f64;
                let success_avg = success as f64 / simulations as f64;
                tx.send((lambda, alpha, success_avg, end_nodes_avg)).unwrap();
            });
        }
    }
    pool.join();
    drop(tx);
    for (lambda, alpha, success, end_nodes_average) in rx {
        simulation_results.push(SimulationResultAlpha{lambda, alpha, success_average: success, end_nodes_average});
    }

    //sort by alpha and lambda
    simulation_results.sort_by(|a, b| {
        if a.lambda == b.lambda {
            a.alpha.partial_cmp(&b.alpha).unwrap()
        } else {
            a.lambda.partial_cmp(&b.lambda).unwrap()
        }
    });
    return simulation_results;
}






fn print_results<S: Display>(results: &Vec<S>) {
    for result in results {
        println!("{}", result);
    }
}

fn print_results_to_csv_file<S:WriteToCsv>(results: &Vec<S>, file_name: &str) {
    let mut wtr = csv::Writer::from_path(file_name).unwrap();
    S::write_header_to_csv(&mut wtr);
    for result in results {
        result.write_to_csv(&mut wtr);
    }
    wtr.flush().unwrap();
}

fn main() {
    //let mut sim = Simulation::new(2.0);
    //sim.run(1000000f64);
    //let result = run_simulation(1000, 1000f64, (1.2, 1.7));
    let result = run_simulation_alpha(
                                      100,
                                      1000f64,
                                      (0.8, 1.8),
                                      (0.9, 1.0),
                                      SimAlphaType::OneDB);
    print_results(&result);
    print_results_to_csv_file(&result, "results.csv");
    //let mut sim = sim2d::Simulation::new(1.1, 0.01);
    //sim.run(1000000f64);

}
