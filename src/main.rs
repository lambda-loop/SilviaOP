use std::fs;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

use crossbeam_channel::unbounded as uchan;

use macroquad::prelude::Conf;

mod problem;
mod rendering;

use problem::heuristic;
use problem::heuristic::{
    experiment_specs,
    multi_results,
};
use problem::Problem;


fn main() {
    // -------------------------------------------------------------------------
    // Experiment configuration
    // -------------------------------------------------------------------------

    // Created only once.
    //
    // Arc allows every worker to access the same immutable
    // experiment specification.
    let specs =
        Arc::new(experiment_specs());


    // -------------------------------------------------------------------------
    // Load problems
    // -------------------------------------------------------------------------

    let dir =
        fs::read_dir("data").unwrap();


    let (tp, rp) = uchan();

    let (tr, rr) =
        mpsc::channel();


    for file in dir {
        let file =
            file.unwrap();


        if !file
            .file_type()
            .unwrap()
            .is_file()
        {
            continue;
        }


        let problem_name =
            file
                .file_name()
                .into_string()
                .unwrap();


        if !problem_name.ends_with(".txt") {
            continue;
        }


        let raw_input =
            fs::read_to_string(
                file.path()
            )
            .unwrap();


        let problem =
            Problem::new(
                &raw_input
            );


        tp.send((
            problem_name,
            problem,
        ))
        .unwrap();
    }


    // No more problems will be sent.
    drop(tp);


    // -------------------------------------------------------------------------
    // Workers
    // -------------------------------------------------------------------------

    let mut handles =
        Vec::new();


    for _ in 0..14 {
        let rp =
            rp.clone();

        let tr =
            tr.clone();

        let specs =
            Arc::clone(&specs);


        let handle =
            thread::spawn(move || {
                while let Ok((
                    problem_name,
                    problem,
                )) = rp.recv()
                {
                    let ers: Vec<_> =
                        multi_results(
                            &problem,
                            &problem_name,
                            specs.as_ref(),
                        )
                        .iter()
                        .map(
                            |er| er.to_csv()
                        )
                        .collect();


                    tr.send(ers)
                        .unwrap();
                }
            });


        handles.push(handle);
    }


    // Main thread no longer needs these.
    drop(rp);
    drop(tr);


    // Wait for workers.
    for handle in handles {
        handle
            .join()
            .unwrap();
    }


    // -------------------------------------------------------------------------
    // CSV
    // -------------------------------------------------------------------------

    let header =
        heuristic::EResult::header();


    let mut content =
        Vec::new();


    content.push(
        format!("{header}\n")
    );


    while let Ok(lines) =
        rr.recv()
    {
        for line in lines {
            content.push(
                format!("{line}\n")
            );
        }
    }


    fs::write(
        "out2.csv",
        content.concat(),
    )
    .unwrap();
}


// Kept because you already use it elsewhere for rendering.
pub fn window_conf() -> Conf {
    Conf {
        window_title:
            "Visualizador de Caminho"
                .to_owned(),

        window_width: 800,
        window_height: 600,
        window_resizable: true,

        ..Default::default()
    }
}
